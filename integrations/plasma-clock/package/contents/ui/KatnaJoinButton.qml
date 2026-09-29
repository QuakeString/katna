// SPDX-License-Identifier: GPL-3.0-or-later

import QtQuick

import org.kde.plasma.components as PlasmaComponents

// Joins a Katna event's video call; hidden when it has none.
PlasmaComponents.Button {
    // An event from KatnaAgenda, or null.
    property var event: null

    visible: event !== null && event.joinUrl.startsWith("https://")
    text: i18ndc("katna-clock", "@action:button join the event's video call", "Join")
    icon.name: "camera-video"
    onClicked: Qt.openUrlExternally(event.joinUrl)
}
