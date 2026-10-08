use crate::{
    AbsoluteLength, App, Bounds, DefiniteLength, Edges, GridTemplate, LayoutDirection, Length,
    Pixels, Point, Size, Style, Window, size,
    util::{
        ceil_to_device_pixel, round_half_toward_zero, round_stroke_to_device_pixel,
        round_to_device_pixel,
    },
};
use collections::{FxHashMap, FxHashSet};
use std::{fmt::Debug, ops::Range};
use taffy::{
    TaffyTree, TraversePartialTree as _,
    geometry::{Point as TaffyPoint, Rect as TaffyRect, Size as TaffySize},
    prelude::{max_content, min_content},
    style::AvailableSpace as TaffyAvailableSpace,
    tree::NodeId,
};

#[cfg(feature = "stacker")]
type StackSafe<T> = stacksafe::StackSafe<T>;
#[cfg(not(feature = "stacker"))]
type StackSafe<T> = T;

type MeasureFn =
    dyn FnMut(Size<Option<Pixels>>, Size<AvailableSpace>, &mut Window, &mut App) -> Size<Pixels>;
type NodeMeasureFn = StackSafe<Box<MeasureFn>>;

struct NodeContext {
    measure: NodeMeasureFn,
}
pub struct TaffyLayoutEngine {
    taffy: TaffyTree<NodeContext>,
    absolute_layout_bounds: FxHashMap<LayoutId, Bounds<Pixels>>,
    /// Unrounded absolute border-box top-left per-node coordinate in device pixels.
    absolute_outer_origins: FxHashMap<LayoutId, Point<f32>>,
    computed_layouts: FxHashSet<LayoutId>,
    layout_bounds_scratch_space: Vec<LayoutId>,
    /// Katna: nodes laid out right to left, whose children's bounds are
    /// mirrored (see `LayoutDirection`).
    rtl_nodes: FxHashSet<LayoutId>,
    /// Katna: nodes placed in left-to-right coordinates whatever their
    /// parent's direction (`Styled::placed_ltr`), so never mirrored.
    placed_ltr_nodes: FxHashSet<LayoutId>,
}

const EXPECT_MESSAGE: &str = "we should avoid taffy layout errors by construction if possible";

impl TaffyLayoutEngine {
    pub fn new() -> Self {
        let mut taffy = TaffyTree::new();
        taffy.disable_rounding();
        TaffyLayoutEngine {
            taffy,
            absolute_layout_bounds: FxHashMap::default(),
            absolute_outer_origins: FxHashMap::default(),
            computed_layouts: FxHashSet::default(),
            layout_bounds_scratch_space: Vec::new(),
            rtl_nodes: FxHashSet::default(),
            placed_ltr_nodes: FxHashSet::default(),
        }
    }

    pub fn clear(&mut self) {
        self.taffy.clear();
        self.absolute_layout_bounds.clear();
        self.absolute_outer_origins.clear();
        self.computed_layouts.clear();
        self.rtl_nodes.clear();
        self.placed_ltr_nodes.clear();
    }

    pub fn request_layout(
        &mut self,
        style: Style,
        rem_size: Pixels,
        scale_factor: f32,
        children: &[LayoutId],
        direction: LayoutDirection,
    ) -> LayoutId {
        let taffy_style = style.to_taffy(rem_size, scale_factor);
        let placed_ltr = style.placed_ltr;

        let id: LayoutId = if children.is_empty() {
            self.taffy
                .new_leaf(taffy_style)
                .expect(EXPECT_MESSAGE)
                .into()
        } else {
            self.taffy
                // This is safe because LayoutId is repr(transparent) to taffy::tree::NodeId.
                .new_with_children(taffy_style, LayoutId::to_taffy_slice(children))
                .expect(EXPECT_MESSAGE)
                .into()
        };
        if direction.is_rtl() {
            self.rtl_nodes.insert(id);
        }
        if placed_ltr {
            self.placed_ltr_nodes.insert(id);
        }
        id
    }

    pub fn request_measured_layout(
        &mut self,
        style: Style,
        rem_size: Pixels,
        scale_factor: f32,
        measure: impl FnMut(
            Size<Option<Pixels>>,
            Size<AvailableSpace>,
            &mut Window,
            &mut App,
        ) -> Size<Pixels>
        + 'static,
    ) -> LayoutId {
        let taffy_style = style.to_taffy(rem_size, scale_factor);
        let measure = Box::new(measure) as Box<MeasureFn>;
        #[cfg(feature = "stacker")]
        let measure = StackSafe::new(measure);

        self.taffy
            .new_leaf_with_context(taffy_style, NodeContext { measure })
            .expect(EXPECT_MESSAGE)
            .into()
    }

    /// Treats any `auto` dimension of the given node's style as filling `size`.
    ///
    /// This is applied to window roots before layout so they behave like the
    /// root element on the web, which stretches to fill the initial containing
    /// block (the viewport) unless given an explicit size. Explicitly styled
    /// dimensions are preserved.
    pub fn stretch_auto_size_to_fill(
        &mut self,
        id: LayoutId,
        size: Size<Pixels>,
        scale_factor: f32,
    ) {
        let style = self.taffy.style(id.0).expect(EXPECT_MESSAGE);
        let stretch_width = style.size.width.is_auto();
        let stretch_height = style.size.height.is_auto();
        if !stretch_width && !stretch_height {
            return;
        }
        let mut style = style.clone();
        if stretch_width {
            style.size.width =
                taffy::style::Dimension::length(round_to_device_pixel(size.width.0, scale_factor));
        }
        if stretch_height {
            style.size.height =
                taffy::style::Dimension::length(round_to_device_pixel(size.height.0, scale_factor));
        }
        self.taffy.set_style(id.0, style).expect(EXPECT_MESSAGE);
    }

    // Used to understand performance
    #[allow(dead_code)]
    fn count_all_children(&self, parent: LayoutId) -> anyhow::Result<u32> {
        let mut count = 0;

        for child in self.taffy.children(parent.0)? {
            // Count this child.
            count += 1;

            // Count all of this child's children.
            count += self.count_all_children(LayoutId(child))?
        }

        Ok(count)
    }

    // Used to understand performance
    #[allow(dead_code)]
    fn max_depth(&self, depth: u32, parent: LayoutId) -> anyhow::Result<u32> {
        println!(
            "{parent:?} at depth {depth} has {} children",
            self.taffy.child_count(parent.0)
        );

        let mut max_child_depth = 0;

        for child in self.taffy.children(parent.0)? {
            max_child_depth = std::cmp::max(max_child_depth, self.max_depth(0, LayoutId(child))?);
        }

        Ok(depth + 1 + max_child_depth)
    }

    // Used to understand performance
    #[allow(dead_code)]
    fn get_edges(&self, parent: LayoutId) -> anyhow::Result<Vec<(LayoutId, LayoutId)>> {
        let mut edges = Vec::new();

        for child in self.taffy.children(parent.0)? {
            edges.push((parent, LayoutId(child)));

            edges.extend(self.get_edges(LayoutId(child))?);
        }

        Ok(edges)
    }

    #[cfg_attr(feature = "stacker", stacksafe::stacksafe)]
    pub fn compute_layout(
        &mut self,
        id: LayoutId,
        available_space: Size<AvailableSpace>,
        window: &mut Window,
        cx: &mut App,
    ) {
        // Leaving this here until we have a better instrumentation approach.
        // println!("Laying out {} children", self.count_all_children(id)?);
        // println!("Max layout depth: {}", self.max_depth(0, id)?);

        // Output the edges (branches) of the tree in Mermaid format for visualization.
        // println!("Edges:");
        // for (a, b) in self.get_edges(id)? {
        //     println!("N{} --> N{}", u64::from(a), u64::from(b));
        // }
        //

        if !self.computed_layouts.insert(id) {
            let stack = &mut self.layout_bounds_scratch_space;
            stack.push(id);
            while let Some(id) = stack.pop() {
                self.absolute_layout_bounds.remove(&id);
                self.absolute_outer_origins.remove(&id);
                stack.extend(
                    self.taffy
                        .children(id.into())
                        .expect(EXPECT_MESSAGE)
                        .into_iter()
                        .map(LayoutId::from),
                );
            }
        }

        let scale_factor = window.scale_factor();

        let transform = |v: AvailableSpace| match v {
            AvailableSpace::Definite(pixels) => {
                AvailableSpace::Definite(Pixels(pixels.0 * scale_factor))
            }
            AvailableSpace::MinContent => AvailableSpace::MinContent,
            AvailableSpace::MaxContent => AvailableSpace::MaxContent,
        };
        let available_space = size(
            transform(available_space.width),
            transform(available_space.height),
        );

        self.taffy
            .compute_layout_with_measure(
                id.into(),
                available_space.into(),
                |known_dimensions, available_space, _id, node_context, _style| {
                    let Some(node_context) = node_context else {
                        return taffy::geometry::Size::default();
                    };

                    let known_dimensions = Size {
                        width: known_dimensions.width.map(|e| Pixels(e / scale_factor)),
                        height: known_dimensions.height.map(|e| Pixels(e / scale_factor)),
                    };

                    let available_space: Size<AvailableSpace> = available_space.into();
                    let untransform = |ev: AvailableSpace| match ev {
                        AvailableSpace::Definite(pixels) => {
                            AvailableSpace::Definite(Pixels(pixels.0 / scale_factor))
                        }
                        AvailableSpace::MinContent => AvailableSpace::MinContent,
                        AvailableSpace::MaxContent => AvailableSpace::MaxContent,
                    };
                    let available_space = size(
                        untransform(available_space.width),
                        untransform(available_space.height),
                    );

                    let measured_size: Size<Pixels> =
                        (node_context.measure)(known_dimensions, available_space, window, cx);
                    snap_measured_size_to_device_pixels(measured_size, scale_factor).into()
                },
            )
            .expect(EXPECT_MESSAGE);
    }

    // Pixel snapping
    //
    // Painting primitives at non-integer pixel coordinates produces blurry
    // output. Pixel snapping converts layout coordinates into integer
    // device-pixel coordinates so painted edges land exactly on physical
    // pixel boundaries.
    //
    // Non-integer coordinates can arise for several reasons, including:
    //   - flex distribution, percentages, centering, and text measurement
    //     can produce fractional element sizes and positions;
    //   - at fractional scale factors (for example 125% or 150%), integer
    //     logical-pixel values can map to non-integer device-pixel values.
    //
    // We pixel-snap by rounding in device-pixel space, after multiplying
    // by `scale_factor`, so that snapping targets physical pixels. Bounds
    // are divided by `scale_factor` before being returned to GPUI.
    //
    // Midpoints are rounded toward zero. This is a stylistic choice: a
    // 1-logical-pixel line at 150% scale should render as 1 dp rather than
    // 2 dp.
    //
    // Pixel snapping is done in two phases:
    //
    //  1. Pre-layout metric snapping. Before Taffy computes layout, all
    //     authored absolute lengths are rounded in `to_taffy`. This
    //     includes borders, padding, gaps, and explicit sizes.
    //     Custom-measured leaf nodes have their measured sizes rounded up
    //     to integer device-pixel lengths.
    //
    //  2. Post-layout edge snapping. After Taffy resolves the tree, layout
    //     relationships such as flex shares, grid tracks, percentages, and
    //     centering can produce new fractional edge positions. Boxes now
    //     have edges in absolute coordinates, and snapping must decide
    //     where those edges land on the device-pixel grid.
    //
    // Ideally, post-layout snapping would satisfy:
    //
    //  - Edge closure. Two raw layout edges at the same absolute position
    //    should snap to the same pixel column.
    //  - Translation stability. A component's internal geometry should not
    //    change when it moves to a new absolute position.
    //
    // These goals are in tension because rounding is not associative.
    // The simple local schemes make different tradeoffs:
    //
    //  - Absolute edge rounding gives each window coordinate one answer,
    //    so coincident edges always close globally. But a span's snapped
    //    length is `round(far) - round(near)`, which may change by 1 dp
    //    as its absolute origin moves.
    //
    //  - Parent-relative edge rounding rounds each child inside its
    //    parent's coordinate space. This guarantees translation stability,
    //    but a shared edge reached through different parents can
    //    accumulate different rounding, causing non-closure between
    //    cousins.
    //
    //  - Length rounding rounds each width, height, and thickness
    //    independently and then places boxes from those rounded lengths.
    //    Sizes stay stable under translation, but neighboring boxes derive
    //    their shared boundary from different sources, so closure is not
    //    guaranteed.
    //
    // We apply absolute edge rounding for each element's outer box in
    // post-layout rounding to preserve closure. Border and padding widths
    // are not touched by post-layout rounding; they keep their pre-layout
    // rounded value so that they remain stable under translation.
    //
    // This gives both closure and translation stability in the case that
    // all local metrics are integer device-pixel lengths. Pre-layout
    // rounding covers that in most cases. The exception is metrics
    // resolved by layout relationships, such as percentages. Outer box
    // edges will still close globally, and painted border widths are still
    // snapped independently, but the raw content-box origin can carry a
    // 1dp residual into descendants.

    pub fn layout_bounds(&mut self, id: LayoutId, scale_factor: f32) -> Bounds<Pixels> {
        if let Some(layout) = self.absolute_layout_bounds.get(&id).cloned() {
            return layout;
        }

        let layout = self.taffy.layout(id.into()).expect(EXPECT_MESSAGE);
        let mut layout_location = layout.location;
        let layout_size = layout.size;
        let parent = self.taffy.parent(id.0);

        let absolute_outer_origin = match parent {
            Some(parent_id) => {
                let parent_id = LayoutId::from(parent_id);
                self.layout_bounds(parent_id, scale_factor);
                // Katna: in a right-to-left parent, mirror the child's x
                // inside the parent's border box, so rows run from the
                // right and left padding, margins and insets act on the
                // right.
                if self.rtl_nodes.contains(&parent_id) && !self.placed_ltr_nodes.contains(&id) {
                    let parent_width = self
                        .taffy
                        .layout(parent_id.into())
                        .expect(EXPECT_MESSAGE)
                        .size
                        .width;
                    layout_location.x = parent_width - layout_location.x - layout_size.width;
                }
                let parent_origin = *self
                    .absolute_outer_origins
                    .get(&parent_id)
                    .expect("parent absolute outer origin should be cached");
                parent_origin + Point::from(layout_location)
            }
            None => Point::from(layout_location),
        };
        self.absolute_outer_origins
            .insert(id, absolute_outer_origin);

        let absolute_far = absolute_outer_origin + Point::from(Size::from(layout_size));
        let snapped_bounds = Bounds::from_corners(
            absolute_outer_origin.map(round_half_toward_zero),
            absolute_far.map(round_half_toward_zero),
        );

        let bounds = (snapped_bounds / scale_factor).map(Pixels);
        self.absolute_layout_bounds.insert(id, bounds);
        bounds
    }
}

/// A unique identifier for a layout node, generated when requesting a layout from Taffy
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
#[repr(transparent)]
pub struct LayoutId(NodeId);

impl LayoutId {
    fn to_taffy_slice(node_ids: &[Self]) -> &[taffy::NodeId] {
        // SAFETY: LayoutId is repr(transparent) to taffy::tree::NodeId.
        unsafe { std::mem::transmute::<&[LayoutId], &[taffy::NodeId]>(node_ids) }
    }
}

impl std::hash::Hash for LayoutId {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        u64::from(self.0).hash(state);
    }
}

impl From<NodeId> for LayoutId {
    fn from(node_id: NodeId) -> Self {
        Self(node_id)
    }
}

impl From<LayoutId> for NodeId {
    fn from(layout_id: LayoutId) -> NodeId {
        layout_id.0
    }
}

fn snap_measured_size_to_device_pixels(size: Size<Pixels>, scale_factor: f32) -> Size<f32> {
    size.map(|d| ceil_to_device_pixel(d.0.max(0.0), scale_factor))
}

fn border_widths_to_taffy(
    widths: &Edges<AbsoluteLength>,
    rem_size: Pixels,
    scale_factor: f32,
) -> TaffyRect<taffy::style::LengthPercentage> {
    let snap = |w: &AbsoluteLength| {
        taffy::style::LengthPercentage::length(round_stroke_to_device_pixel(
            w.to_pixels(rem_size).0,
            scale_factor,
        ))
    };
    TaffyRect {
        top: snap(&widths.top),
        right: snap(&widths.right),
        bottom: snap(&widths.bottom),
        left: snap(&widths.left),
    }
}

trait ToTaffy<Output> {
    fn to_taffy(&self, rem_size: Pixels, scale_factor: f32) -> Output;
}

impl ToTaffy<taffy::style::Style> for Style {
    fn to_taffy(&self, rem_size: Pixels, scale_factor: f32) -> taffy::style::Style {
        use taffy::style_helpers::{fr, length, minmax, repeat};

        fn to_grid_line(
            placement: &Range<crate::GridPlacement>,
        ) -> taffy::Line<taffy::GridPlacement> {
            taffy::Line {
                start: placement.start.into(),
                end: placement.end.into(),
            }
        }

        fn to_grid_repeat<T: taffy::style::CheapCloneStr>(
            unit: &Option<GridTemplate>,
        ) -> Vec<taffy::GridTemplateComponent<T>> {
            unit.map(|template| {
                match template.min_size {
                    // grid-template-*: repeat(<number>, minmax(0, 1fr));
                    crate::GridTemplateMinSize::Zero => {
                        vec![repeat(
                            template.repeat,
                            vec![minmax(length(0.0_f32), fr(1.0_f32))],
                        )]
                    }
                    // grid-template-*: repeat(<number>, minmax(min-content, 1fr));
                    crate::GridTemplateMinSize::MinContent => {
                        vec![repeat(
                            template.repeat,
                            vec![minmax(min_content(), fr(1.0_f32))],
                        )]
                    }
                    // grid-template-*: repeat(<number>, minmax(0, max-content))
                    crate::GridTemplateMinSize::MaxContent => {
                        vec![repeat(
                            template.repeat,
                            vec![minmax(length(0.0_f32), max_content())],
                        )]
                    }
                }
            })
            .unwrap_or_default()
        }

        taffy::style::Style {
            display: self.display.into(),
            overflow: self.overflow.into(),
            scrollbar_width: self.scrollbar_width.to_taffy(rem_size, scale_factor),
            position: self.position.into(),
            inset: self.inset.to_taffy(rem_size, scale_factor),
            size: self.size.to_taffy(rem_size, scale_factor),
            min_size: self.min_size.to_taffy(rem_size, scale_factor),
            max_size: self.max_size.to_taffy(rem_size, scale_factor),
            aspect_ratio: self.aspect_ratio,
            margin: self.margin.to_taffy(rem_size, scale_factor),
            padding: self.padding.to_taffy(rem_size, scale_factor),
            border: border_widths_to_taffy(&self.border_widths, rem_size, scale_factor),
            align_items: self.align_items.map(|x| x.into()),
            align_self: self.align_self.map(|x| x.into()),
            align_content: self.align_content.map(|x| x.into()),
            justify_content: self.justify_content.map(|x| x.into()),
            gap: self.gap.to_taffy(rem_size, scale_factor),
            flex_direction: self.flex_direction.into(),
            flex_wrap: self.flex_wrap.into(),
            flex_basis: self.flex_basis.to_taffy(rem_size, scale_factor),
            flex_grow: self.flex_grow,
            flex_shrink: self.flex_shrink,
            grid_template_rows: to_grid_repeat(&self.grid_rows),
            grid_template_columns: to_grid_repeat(&self.grid_cols),
            grid_row: self
                .grid_location
                .as_ref()
                .map(|location| to_grid_line(&location.row))
                .unwrap_or_default(),
            grid_column: self
                .grid_location
                .as_ref()
                .map(|location| to_grid_line(&location.column))
                .unwrap_or_default(),
            ..Default::default()
        }
    }
}

impl ToTaffy<f32> for AbsoluteLength {
    fn to_taffy(&self, rem_size: Pixels, scale_factor: f32) -> f32 {
        round_to_device_pixel(self.to_pixels(rem_size).0, scale_factor)
    }
}

impl ToTaffy<taffy::style::LengthPercentageAuto> for Length {
    fn to_taffy(
        &self,
        rem_size: Pixels,
        scale_factor: f32,
    ) -> taffy::prelude::LengthPercentageAuto {
        match self {
            Length::Definite(length) => length.to_taffy(rem_size, scale_factor),
            Length::Auto => taffy::prelude::LengthPercentageAuto::auto(),
        }
    }
}

impl ToTaffy<taffy::style::Dimension> for Length {
    fn to_taffy(&self, rem_size: Pixels, scale_factor: f32) -> taffy::prelude::Dimension {
        match self {
            Length::Definite(length) => length.to_taffy(rem_size, scale_factor),
            Length::Auto => taffy::prelude::Dimension::auto(),
        }
    }
}

impl ToTaffy<taffy::style::LengthPercentage> for DefiniteLength {
    fn to_taffy(&self, rem_size: Pixels, scale_factor: f32) -> taffy::style::LengthPercentage {
        match self {
            DefiniteLength::Absolute(length) => length.to_taffy(rem_size, scale_factor),
            DefiniteLength::Fraction(fraction) => {
                taffy::style::LengthPercentage::percent(*fraction)
            }
        }
    }
}

impl ToTaffy<taffy::style::LengthPercentageAuto> for DefiniteLength {
    fn to_taffy(&self, rem_size: Pixels, scale_factor: f32) -> taffy::style::LengthPercentageAuto {
        match self {
            DefiniteLength::Absolute(length) => length.to_taffy(rem_size, scale_factor),
            DefiniteLength::Fraction(fraction) => {
                taffy::style::LengthPercentageAuto::percent(*fraction)
            }
        }
    }
}

impl ToTaffy<taffy::style::Dimension> for DefiniteLength {
    fn to_taffy(&self, rem_size: Pixels, scale_factor: f32) -> taffy::style::Dimension {
        match self {
            DefiniteLength::Absolute(length) => length.to_taffy(rem_size, scale_factor),
            DefiniteLength::Fraction(fraction) => taffy::style::Dimension::percent(*fraction),
        }
    }
}

impl ToTaffy<taffy::style::LengthPercentage> for AbsoluteLength {
    fn to_taffy(&self, rem_size: Pixels, scale_factor: f32) -> taffy::style::LengthPercentage {
        taffy::style::LengthPercentage::length(self.to_taffy(rem_size, scale_factor))
    }
}

impl ToTaffy<taffy::style::LengthPercentageAuto> for AbsoluteLength {
    fn to_taffy(&self, rem_size: Pixels, scale_factor: f32) -> taffy::style::LengthPercentageAuto {
        taffy::style::LengthPercentageAuto::length(self.to_taffy(rem_size, scale_factor))
    }
}

impl ToTaffy<taffy::style::Dimension> for AbsoluteLength {
    fn to_taffy(&self, rem_size: Pixels, scale_factor: f32) -> taffy::style::Dimension {
        taffy::style::Dimension::length(self.to_taffy(rem_size, scale_factor))
    }
}

impl<T, T2> From<TaffyPoint<T>> for Point<T2>
where
    T: Into<T2>,
    T2: Clone + Debug + Default + PartialEq,
{
    fn from(point: TaffyPoint<T>) -> Point<T2> {
        Point {
            x: point.x.into(),
            y: point.y.into(),
        }
    }
}

impl<T, T2> From<Point<T>> for TaffyPoint<T2>
where
    T: Into<T2> + Clone + Debug + Default + PartialEq,
{
    fn from(val: Point<T>) -> Self {
        TaffyPoint {
            x: val.x.into(),
            y: val.y.into(),
        }
    }
}

impl<T, U> ToTaffy<TaffySize<U>> for Size<T>
where
    T: ToTaffy<U> + Clone + Debug + Default + PartialEq,
{
    fn to_taffy(&self, rem_size: Pixels, scale_factor: f32) -> TaffySize<U> {
        TaffySize {
            width: self.width.to_taffy(rem_size, scale_factor),
            height: self.height.to_taffy(rem_size, scale_factor),
        }
    }
}

impl<T, U> ToTaffy<TaffyRect<U>> for Edges<T>
where
    T: ToTaffy<U> + Clone + Debug + Default + PartialEq,
{
    fn to_taffy(&self, rem_size: Pixels, scale_factor: f32) -> TaffyRect<U> {
        TaffyRect {
            top: self.top.to_taffy(rem_size, scale_factor),
            right: self.right.to_taffy(rem_size, scale_factor),
            bottom: self.bottom.to_taffy(rem_size, scale_factor),
            left: self.left.to_taffy(rem_size, scale_factor),
        }
    }
}

impl<T, U> From<TaffySize<T>> for Size<U>
where
    T: Into<U>,
    U: Clone + Debug + Default + PartialEq,
{
    fn from(taffy_size: TaffySize<T>) -> Self {
        Size {
            width: taffy_size.width.into(),
            height: taffy_size.height.into(),
        }
    }
}

impl<T, U> From<Size<T>> for TaffySize<U>
where
    T: Into<U> + Clone + Debug + Default + PartialEq,
{
    fn from(size: Size<T>) -> Self {
        TaffySize {
            width: size.width.into(),
            height: size.height.into(),
        }
    }
}

/// The space available for an element to be laid out in
#[derive(Copy, Clone, Default, Debug, Eq, PartialEq)]
pub enum AvailableSpace {
    /// The amount of space available is the specified number of pixels
    Definite(Pixels),
    /// The amount of space available is indefinite and the node should be laid out under a min-content constraint
    #[default]
    MinContent,
    /// The amount of space available is indefinite and the node should be laid out under a max-content constraint
    MaxContent,
}

impl AvailableSpace {
    /// Returns a `Size` with both width and height set to `AvailableSpace::MinContent`.
    ///
    /// This function is useful when you want to create a `Size` with the minimum content constraints
    /// for both dimensions.
    ///
    /// # Examples
    ///
    /// ```
    /// use gpui::AvailableSpace;
    /// let min_content_size = AvailableSpace::min_size();
    /// assert_eq!(min_content_size.width, AvailableSpace::MinContent);
    /// assert_eq!(min_content_size.height, AvailableSpace::MinContent);
    /// ```
    pub const fn min_size() -> Size<Self> {
        Size {
            width: Self::MinContent,
            height: Self::MinContent,
        }
    }
}

impl From<AvailableSpace> for TaffyAvailableSpace {
    fn from(space: AvailableSpace) -> TaffyAvailableSpace {
        match space {
            AvailableSpace::Definite(Pixels(value)) => TaffyAvailableSpace::Definite(value),
            AvailableSpace::MinContent => TaffyAvailableSpace::MinContent,
            AvailableSpace::MaxContent => TaffyAvailableSpace::MaxContent,
        }
    }
}

impl From<TaffyAvailableSpace> for AvailableSpace {
    fn from(space: TaffyAvailableSpace) -> AvailableSpace {
        match space {
            TaffyAvailableSpace::Definite(value) => AvailableSpace::Definite(Pixels(value)),
            TaffyAvailableSpace::MinContent => AvailableSpace::MinContent,
            TaffyAvailableSpace::MaxContent => AvailableSpace::MaxContent,
        }
    }
}

impl From<Pixels> for AvailableSpace {
    fn from(pixels: Pixels) -> Self {
        AvailableSpace::Definite(pixels)
    }
}

impl From<Size<Pixels>> for Size<AvailableSpace> {
    fn from(size: Size<Pixels>) -> Self {
        Size {
            width: AvailableSpace::Definite(size.width),
            height: AvailableSpace::Definite(size.height),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn border_widths_to_taffy_use_stroke_snapping() {
        let border_widths = Edges {
            top: Pixels(0.0).into(),
            right: Pixels(0.4).into(),
            bottom: Pixels(0.5).into(),
            left: Pixels(1.6).into(),
        };
        let taffy_border = border_widths_to_taffy(&border_widths, Pixels(16.0), 1.0);

        assert_eq!(
            taffy_border.top,
            taffy::style::LengthPercentage::length(0.0)
        );
        assert_eq!(
            taffy_border.right,
            taffy::style::LengthPercentage::length(1.0)
        );
        assert_eq!(
            taffy_border.bottom,
            taffy::style::LengthPercentage::length(1.0)
        );
        assert_eq!(
            taffy_border.left,
            taffy::style::LengthPercentage::length(2.0)
        );
    }
}

/// Katna: right-to-left layout (`LayoutDirection`).
#[cfg(test)]
mod direction_tests {
    use crate::{
        AnyElement, Bounds, Corners, Edges, IntoElement as _, LayoutDirection, ParentElement as _,
        Pixels, Style, Styled as _, TestAppContext, TextAlign, canvas, div, point, px, size,
    };
    use std::{cell::Cell, rc::Rc};

    type Probe = Rc<Cell<Bounds<Pixels>>>;

    /// A box of the given width that records its bounds.
    fn probe(width: f32, probe: &Probe) -> AnyElement {
        let probe = probe.clone();
        canvas(move |bounds, _, _| probe.set(bounds), |_, _, _, _| {})
            .w(px(width))
            .h(px(10.))
            .into_any_element()
    }

    fn x_range(probe: &Probe) -> (f32, f32) {
        let bounds = probe.get();
        (bounds.left().into(), bounds.right().into())
    }

    /// Draws a 100 px wide flex row: `first` (30 px), `second` (20 px) and
    /// a 40 px left-to-right row of `inner_a` and `inner_b` (10 px each),
    /// with 4 px of left padding.
    fn draw_row(
        cx: &mut TestAppContext,
        window_direction: LayoutDirection,
        root: impl FnOnce(crate::Div) -> crate::Div,
    ) -> [Probe; 4] {
        let probes: [Probe; 4] = Default::default();
        let cx = cx.add_empty_window();
        cx.update(|window, _| window.set_layout_direction(window_direction));
        let [first, second, inner_a, inner_b] = probes.clone();
        cx.draw(
            point(px(0.), px(0.)),
            size(px(100.), px(10.)),
            move |_, _| {
                root(div().w(px(100.)).h(px(10.)).flex().flex_row().pl(px(4.)))
                    .child(probe(30., &first))
                    .child(probe(20., &second))
                    .child(
                        div()
                            .layout_ltr()
                            .w(px(40.))
                            .flex()
                            .flex_row()
                            .child(probe(10., &inner_a))
                            .child(probe(10., &inner_b)),
                    )
            },
        );
        probes
    }

    #[gpui::test]
    fn left_to_right_is_unchanged(cx: &mut TestAppContext) {
        let [first, second, inner_a, inner_b] = draw_row(cx, LayoutDirection::Ltr, |root| root);
        assert_eq!(x_range(&first), (4., 34.));
        assert_eq!(x_range(&second), (34., 54.));
        assert_eq!(x_range(&inner_a), (54., 64.));
        assert_eq!(x_range(&inner_b), (64., 74.));
    }

    #[gpui::test]
    fn right_to_left_root_mirrors_children(cx: &mut TestAppContext) {
        let [first, second, inner_a, inner_b] =
            draw_row(cx, LayoutDirection::Ltr, |root| root.layout_rtl());
        // The row starts on the right, and the left padding is on the right.
        assert_eq!(x_range(&first), (66., 96.));
        assert_eq!(x_range(&second), (46., 66.));
        // The left-to-right row is placed from the right (6..46) but runs
        // left to right inside.
        assert_eq!(x_range(&inner_a), (6., 16.));
        assert_eq!(x_range(&inner_b), (16., 26.));
    }

    #[gpui::test]
    fn right_to_left_window_mirrors_children(cx: &mut TestAppContext) {
        let [first, second, inner_a, inner_b] = draw_row(cx, LayoutDirection::Rtl, |root| root);
        assert_eq!(x_range(&first), (66., 96.));
        assert_eq!(x_range(&second), (46., 66.));
        assert_eq!(x_range(&inner_a), (6., 16.));
        assert_eq!(x_range(&inner_b), (16., 26.));
    }

    #[gpui::test]
    fn layout_ltr_restores_left_to_right(cx: &mut TestAppContext) {
        let [first, second, inner_a, inner_b] =
            draw_row(cx, LayoutDirection::Rtl, |root| root.layout_ltr());
        assert_eq!(x_range(&first), (4., 34.));
        assert_eq!(x_range(&second), (34., 54.));
        assert_eq!(x_range(&inner_a), (54., 64.));
        assert_eq!(x_range(&inner_b), (64., 74.));
    }

    #[gpui::test]
    fn placed_ltr_keeps_its_own_inset_only(cx: &mut TestAppContext) {
        let probes: [Probe; 2] = Default::default();
        let cx = cx.add_empty_window();
        cx.update(|window, _| window.set_layout_direction(LayoutDirection::Rtl));
        let [first, second] = probes.clone();
        cx.draw(point(px(0.), px(0.)), size(px(100.), px(10.)), move |_, _| {
            div().relative().w(px(100.)).h(px(10.)).child(
                div()
                    .absolute()
                    .placed_ltr()
                    .left(px(10.))
                    .w(px(30.))
                    .flex()
                    .flex_row()
                    .child(probe(10., &first))
                    .child(probe(10., &second)),
            )
        });
        // At 10 px from the left, its row still running from the right.
        assert_eq!(x_range(&probes[0]), (30., 40.));
        assert_eq!(x_range(&probes[1]), (20., 30.));
    }

    #[test]
    fn text_align_follows_direction() {
        use LayoutDirection::{Ltr, Rtl};
        assert_eq!(TextAlign::Left.resolve(Ltr), TextAlign::Left);
        assert_eq!(TextAlign::Right.resolve(Ltr), TextAlign::Right);
        assert_eq!(TextAlign::Left.resolve(Rtl), TextAlign::Right);
        assert_eq!(TextAlign::Right.resolve(Rtl), TextAlign::Left);
        assert_eq!(TextAlign::Center.resolve(Rtl), TextAlign::Center);
    }

    #[test]
    fn mirrored_style_swaps_borders_and_corners() {
        let mut style = Style::default();
        style.border_widths = Edges {
            left: px(1.).into(),
            right: px(2.).into(),
            ..Edges::default()
        };
        style.corner_radii = Corners {
            top_left: px(3.).into(),
            bottom_left: px(4.).into(),
            ..Corners::default()
        };
        style.box_shadow = vec![crate::BoxShadow {
            color: crate::black(),
            offset: point(px(3.), px(5.)),
            blur_radius: px(2.),
            spread_radius: px(0.),
            inset: false,
        }];
        let mirrored = style.mirrored();
        assert_eq!(mirrored.box_shadow[0].offset, point(px(-3.), px(5.)));
        assert_eq!(mirrored.border_widths.left, px(2.).into());
        assert_eq!(mirrored.border_widths.right, px(1.).into());
        assert_eq!(mirrored.corner_radii.top_right, px(3.).into());
        assert_eq!(mirrored.corner_radii.bottom_right, px(4.).into());
        assert_eq!(mirrored.corner_radii.top_left, px(0.).into());
    }

    /// A right-to-left window that draws what its function makes.
    struct RtlView(Box<dyn Fn() -> AnyElement>);

    impl crate::Render for RtlView {
        fn render(
            &mut self,
            window: &mut crate::Window,
            _: &mut crate::Context<Self>,
        ) -> impl crate::IntoElement {
            window.set_layout_direction(LayoutDirection::Rtl);
            (self.0)()
        }
    }

    /// Opens a right-to-left window drawing `element`.
    fn draw_rtl(
        cx: &mut TestAppContext,
        element: impl Fn() -> AnyElement + 'static,
    ) -> &mut crate::VisualTestContext {
        let (_, cx) = cx.add_window_view(|_, _| RtlView(Box::new(element)));
        cx.run_until_parked();
        cx
    }

    fn redraw(cx: &mut crate::VisualTestContext) {
        cx.update(|window, _| window.refresh());
        cx.run_until_parked();
    }

    #[gpui::test]
    fn right_to_left_row_scrolls_toward_the_right(cx: &mut TestAppContext) {
        use crate::{InteractiveElement as _, StatefulInteractiveElement as _};
        let handle = crate::ScrollHandle::new();
        let content: Probe = Default::default();
        let (row, probe_content) = (handle.clone(), content.clone());
        let cx = draw_rtl(cx, move || {
            div()
                .id("row")
                .w(px(100.))
                .h(px(10.))
                .flex()
                .overflow_x_scroll()
                .track_scroll(&row)
                .child(div().flex_none().child(probe(300., &probe_content)))
                .into_any_element()
        });
        // The row starts at the right edge and runs out past the left.
        assert_eq!(x_range(&content), (-200., 100.));
        assert_eq!(handle.max_offset().x, px(200.));
        // Scrolling on moves it right, up to the most it can.
        handle.set_offset(point(px(50.), px(0.)));
        redraw(cx);
        assert_eq!(x_range(&content), (-150., 150.));
        handle.set_offset(point(px(500.), px(0.)));
        redraw(cx);
        assert_eq!(handle.offset().x, px(200.));
        // Past its start it cannot go.
        handle.set_offset(point(px(-50.), px(0.)));
        redraw(cx);
        assert_eq!(handle.offset().x, px(0.));
        assert_eq!(x_range(&content), (-200., 100.));
    }

    #[gpui::test]
    fn right_to_left_uniform_list_starts_at_the_right(cx: &mut TestAppContext) {
        let item: Probe = Default::default();
        let probe_item = item.clone();
        draw_rtl(cx, move || {
            let probe_item = probe_item.clone();
            div()
                .w(px(100.))
                .h(px(40.))
                .child(
                    crate::uniform_list("list", 3, move |range, _, _| {
                        range
                            .map(|ix| {
                                if ix == 0 {
                                    probe(30., &probe_item)
                                } else {
                                    div().w(px(30.)).h(px(10.)).into_any_element()
                                }
                            })
                            .collect::<Vec<_>>()
                    })
                    .with_horizontal_sizing_behavior(
                        crate::ListHorizontalSizingBehavior::Unconstrained,
                    )
                    .pl(px(4.))
                    .size_full(),
                )
                .into_any_element()
        });
        // Against the right edge, inside the start padding.
        assert_eq!(x_range(&item), (66., 96.));
    }

    #[gpui::test]
    fn right_to_left_list_starts_at_the_right(cx: &mut TestAppContext) {
        let item: Probe = Default::default();
        let state = crate::ListState::new(2, crate::ListAlignment::Top, px(100.));
        let probe_item = item.clone();
        draw_rtl(cx, move || {
            let probe_item = probe_item.clone();
            div()
                .w(px(100.))
                .h(px(40.))
                .child(
                    crate::list(state.clone(), move |ix, _, _| {
                        let row = div().flex().h(px(10.));
                        if ix == 0 {
                            row.child(probe(30., &probe_item)).into_any_element()
                        } else {
                            row.into_any_element()
                        }
                    })
                    .size_full(),
                )
                .into_any_element()
        });
        // A full-width row, whose first box is at its right.
        assert_eq!(x_range(&item), (70., 100.));
    }
}
