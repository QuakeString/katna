#!/bin/sh
# SPDX-License-Identifier: GPL-3.0-or-later
# Rewrites katna-clock.pot from the strings in Katna Digital Clock (Plasma)
# and Katna's GNOME Shell extension. Run it after changing their text.
set -eu
export LC_ALL=C.UTF-8
cd "$(dirname "$0")/.."
# Plasma: i18nd("katna-clock", …) and i18ndc("katna-clock", context, …);
# the copied upstream files use Plasma's own domain, so only the Katna*
# files are read.
xgettext --from-code=UTF-8 --language=JavaScript --add-comments=Translators \
  --keyword= --keyword=i18nd:2 --keyword=i18ndc:2c,3 \
  --package-name=katna-clock --msgid-bugs-address=https://github.com/QuakeString/katna/issues \
  --omit-header --sort-by-file \
  -o po/plasma.pot plasma-clock/package/contents/ui/Katna*.qml
# GNOME: gettext as _() and pgettext(context, …).
xgettext --from-code=UTF-8 --language=JavaScript --add-comments=Translators \
  --keyword= --keyword=_ --keyword=pgettext:1c,2 \
  --omit-header --sort-by-file \
  -o po/gnome.pot gnome-shell-extension/*/extension.js
{
  printf '# SPDX-License-Identifier: GPL-3.0-or-later\n'
  printf '# Katna Digital Clock and the GNOME Shell extension. Made by update-pot.sh.\n'
  printf 'msgid ""\nmsgstr ""\n"Content-Type: text/plain; charset=UTF-8\\n"\n\n'
  msgcat --use-first --sort-by-file po/plasma.pot po/gnome.pot | sed '/^#: /s/:[0-9]*//g'
} >po/katna-clock.pot
rm po/plasma.pot po/gnome.pot
