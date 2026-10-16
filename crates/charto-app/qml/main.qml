import QtQuick
import QtQuick.Controls
import QtQuick.Dialogs
import QtQuick.Layouts
import QtQml
import org.charto.app

ApplicationWindow {
    id: win
    width: 1280
    height: 820
    visible: true
    title: "Charto 3.1" + (snap.file !== "" ? " – " + snap.file : "") + (snap.dirty ? " *" : "")

    ChartBackend { id: backend }

    // a file given on the command line (or double-clicked in a file manager) is opened at start-up
    Component.onCompleted: {
        var f = backend.startupFile()
        if (f !== "")
            requestOpen(f)
    }

    // the whole editor state, republished by Rust after every action
    property var snap: JSON.parse(backend.snapshot)
    property var symbolPalette: JSON.parse(backend.paletteJson())
    property real zoom: 1.0
    // while printing: selection marks are hidden and symbols are rendered at print resolution
    property bool printing: false
    property real printScale: 1.0
    property bool printBusy: false
    // action waiting for the answer of the "unsaved changes" dialog
    property var pendingAction: null
    property bool closing: false
    readonly property real cw: snap.cellW * zoom
    readonly property real ch: snap.cellH * zoom
    readonly property real leftMargin: 80
    readonly property real rightMargin: 80
    readonly property real bottomMargin: 34

    function zoomBy(f) {
        zoom = Math.max(0.3, Math.min(4.0, zoom * f))
    }

    function localPath(u) {
        var s = u.toString().replace(/^file:\/\//, "")
        if (/^\/[A-Za-z]:/.test(s))
            s = s.substring(1)
        return decodeURIComponent(s)
    }

    // Translation: the texts come from the .po files built into the program (see po/)
    function tr(text) { return backend.translate(text) }

    function save() {
        if (snap.path !== "")
            backend.saveFile(snap.path)
        else
            chooseFile("save")
    }

    // ---- file dialogs ------------------------------------------------------
    // The desktop's own dialog (freedesktop FileChooser portal) is asked for
    // through the backend; if the desktop offers none, Qt's own dialog below
    // is used instead.
    property bool fileDialogBusy: false

    function chooseFile(purpose) {
        if (fileDialogBusy)
            return
        fileDialogBusy = true
        backend.chooseFile(purpose, snap.path)
    }

    function fallbackDialog(purpose) {
        if (purpose === "open")
            openDialog.open()
        else if (purpose === "save")
            saveDialog.open()
        else if (purpose === "exportPng")
            exportPngDialog.open()
        else if (purpose === "exportSvg")
            exportSvgDialog.open()
    }

    function fileChosen(purpose, path) {
        if (purpose === "open") {
            requestOpen(path)
        } else if (purpose === "save") {
            backend.saveFile(path)
            if (backend.status === "")
                runPending()
            else
                pendingAction = null
        } else if (purpose === "exportPng") {
            exportPng(path)
        } else if (purpose === "exportSvg") {
            backend.exportSvg(path)
        }
    }

    function fileCancelled(purpose) {
        if (purpose === "save")
            pendingAction = null
    }

    Connections {
        target: backend
        function onPickedChanged() {
            var r = JSON.parse(backend.picked)
            win.fileDialogBusy = false
            if (r.state === "chosen")
                win.fileChosen(r.purpose, r.path)
            else if (r.state === "cancelled")
                win.fileCancelled(r.purpose)
            else
                win.fallbackDialog(r.purpose)
        }
    }

    // Renders the whole chart (not just the visible part) to an image and
    // passes the grab result to `callback`.
    function renderChart(callback) {
        if (printBusy)
            return
        printBusy = true
        var w = canvas.width
        var h = canvas.height
        printScale = Math.max(1, Math.min(6, 3600 / Math.max(w, h)))
        printing = true
        canvas.grabToImage(function (result) {
            printing = false
            printScale = 1
            printBusy = false
            callback(result)
        }, Qt.size(Math.round(w * printScale), Math.round(h * printScale)))
    }

    // hands the rendering to the backend, which shows the desktop print dialog
    function printChart() {
        renderChart(function (result) {
            var path = backend.printTempPath()
            if (result.saveToFile(path))
                backend.printImage(path, snap.file !== "" ? snap.file : tr("Charto pattern"))
            else
                backend.reportStatus(tr("Could not render the chart for printing"))
        })
    }

    function exportPng(path) {
        renderChart(function (result) {
            var target = /\.[A-Za-z0-9]+$/.test(path) ? path : path + ".png"
            backend.reportStatus(result.saveToFile(target)
                                 ? tr("Exported %1").arg(target)
                                 : tr("Could not write %1").arg(target))
        })
    }

    // runs `action` now, or after the user has dealt with unsaved changes
    function guardUnsaved(action) {
        if (!snap.dirty) {
            action()
            return
        }
        pendingAction = action
        unsavedDialog.open()
    }

    function runPending() {
        var action = pendingAction
        pendingAction = null
        if (action)
            action()
    }

    // opens a pattern, offering the recovered version if one exists
    function requestOpen(path) {
        if (backend.hasRecovery(path)) {
            recoveryDialog.path = path
            recoveryDialog.open()
        } else {
            backend.openFile(path)
        }
    }

    onClosing: (close) => {
        if (!closing && snap.dirty) {
            close.accepted = false
            guardUnsaved(function () { closing = true; win.close() })
        } else {
            backend.endSession()
        }
    }

    // ---- menus ------------------------------------------------------------

    menuBar: MenuBar {
        Menu {
            title: tr("&File")
            Action {
                text: tr("&New…")
                icon.name: "document-new"
                onTriggered: win.guardUnsaved(function () { newDialog.open() })
            }
            Action {
                text: tr("&Open…")
                icon.name: "document-open"
                onTriggered: win.guardUnsaved(function () { win.chooseFile("open") })
            }
            Menu {
                id: recentMenu
                title: tr("Open &Recent")
                enabled: snap.recent.length > 0
                Instantiator {
                    model: snap.recent
                    delegate: MenuItem {
                        required property var modelData
                        text: modelData.name
                        onTriggered: win.guardUnsaved(function () { win.requestOpen(modelData.path) })
                    }
                    onObjectAdded: (index, object) => recentMenu.insertItem(index, object)
                    onObjectRemoved: (index, object) => recentMenu.removeItem(object)
                }
                MenuSeparator {}
                Action { text: tr("&Clear list"); onTriggered: backend.clearRecentFiles() }
            }
            Action { text: tr("&Save"); icon.name: "document-save"; onTriggered: win.save() }
            Action { text: tr("Save &As…"); icon.name: "document-save-as"; onTriggered: win.chooseFile("save") }
            MenuSeparator {}
            Action {
                text: tr("Export as &PNG…")
                icon.name: "image-x-generic"
                enabled: !win.printBusy
                onTriggered: win.chooseFile("exportPng")
            }
            Action { text: tr("Export as S&VG…"); icon.name: "image-svg+xml"; onTriggered: win.chooseFile("exportSvg") }
            MenuSeparator {}
            Action {
                text: tr("&Print…")
                icon.name: "document-print"
                enabled: !win.printBusy
                onTriggered: win.printChart()
            }
            MenuSeparator {}
            Action { text: tr("&Quit"); icon.name: "application-exit"; onTriggered: win.close() }
        }
        Menu {
            title: tr("&Edit")
            Action {
                text: snap.canUndo ? tr("&Undo %1").arg(tr(snap.undoText)) : tr("&Undo")
                icon.name: "edit-undo"
                enabled: snap.canUndo
                onTriggered: backend.undo()
            }
            Action {
                text: snap.canRedo ? tr("&Redo %1").arg(tr(snap.redoText)) : tr("&Redo")
                icon.name: "edit-redo"
                enabled: snap.canRedo
                onTriggered: backend.redo()
            }
            MenuSeparator {}
            Action { text: tr("&Copy"); icon.name: "edit-copy"; enabled: snap.selected > 0; onTriggered: backend.copy() }
            Action {
                text: tr("&Paste into selection")
                icon.name: "edit-paste"
                enabled: snap.hasClipboard
                onTriggered: backend.paste(-1, -1)
            }
            MenuSeparator {}
            Action { text: tr("Clear &selection"); onTriggered: backend.clearSelection() }
            Action { text: tr("Apply colour to selection"); enabled: snap.selected > 0; onTriggered: backend.applyColor() }
            MenuSeparator {}
            Action {
                text: tr("&Preferences…")
                icon.name: "preferences-system"
                onTriggered: prefDialog.load()
            }
        }
        Menu {
            title: tr("&Chart")
            Action { text: tr("Insert rows above…"); onTriggered: countDialog.ask("rowsAbove", text) }
            Action { text: tr("Insert rows below…"); onTriggered: countDialog.ask("rowsBelow", text) }
            Action { text: tr("Delete selected rows"); onTriggered: backend.deleteRows() }
            MenuSeparator {}
            Action { text: tr("Insert stitches left…"); onTriggered: countDialog.ask("colsLeft", text) }
            Action { text: tr("Insert stitches right…"); onTriggered: countDialog.ask("colsRight", text) }
            Action { text: tr("Delete selected stitches"); onTriggered: backend.deleteColumns() }
            MenuSeparator {}
            Action { text: tr("Make selected rows a row repeat…"); onTriggered: countDialog.ask("repeat", text) }
            Action { text: tr("Delete row repeat"); onTriggered: backend.deleteRowRepeat() }
        }
        Menu {
            title: tr("&View")
            Action { text: tr("Zoom &in"); icon.name: "zoom-in"; onTriggered: win.zoomBy(1.2) }
            Action { text: tr("Zoom &out"); icon.name: "zoom-out"; onTriggered: win.zoomBy(1 / 1.2) }
            Action { text: tr("&Reset zoom"); icon.name: "zoom-original"; onTriggered: win.zoom = 1.0 }
        }
    }


    // ---- keyboard shortcuts -------------------------------------------------
    // Kept apart from the menu actions so that the menus show no shortcut text.

    Shortcut { sequences: [StandardKey.New]; onActivated: win.guardUnsaved(function () { newDialog.open() }) }
    Shortcut { sequences: [StandardKey.Open]; onActivated: win.guardUnsaved(function () { win.chooseFile("open") }) }
    Shortcut { sequences: [StandardKey.Save]; onActivated: win.save() }
    Shortcut { sequences: [StandardKey.SaveAs]; onActivated: win.chooseFile("save") }
    Shortcut { sequences: [StandardKey.Print]; enabled: !win.printBusy; onActivated: win.printChart() }
    Shortcut { sequences: [StandardKey.Quit]; onActivated: win.close() }
    Shortcut { sequences: [StandardKey.Undo]; enabled: snap.canUndo; onActivated: backend.undo() }
    Shortcut { sequences: [StandardKey.Redo]; enabled: snap.canRedo; onActivated: backend.redo() }
    Shortcut { sequences: [StandardKey.Copy]; enabled: snap.selected > 0; onActivated: backend.copy() }
    Shortcut { sequences: [StandardKey.Paste]; enabled: snap.hasClipboard; onActivated: backend.paste(-1, -1) }
    Shortcut { sequence: "Escape"; onActivated: backend.clearSelection() }
    Shortcut { sequences: [StandardKey.Preferences]; onActivated: prefDialog.load() }
    Shortcut { sequences: [StandardKey.ZoomIn]; onActivated: win.zoomBy(1.2) }
    Shortcut { sequences: [StandardKey.ZoomOut]; onActivated: win.zoomBy(1 / 1.2) }
    Shortcut { sequence: "Ctrl+0"; onActivated: win.zoom = 1.0 }

    // ---- dialogs ----------------------------------------------------------

    FileDialog {
        id: openDialog
        title: tr("Open pattern")
        nameFilters: [tr("Charto patterns (*.spf)"), tr("All files (*)")]
        onAccepted: win.requestOpen(win.localPath(selectedFile))
    }

    FileDialog {
        id: saveDialog
        title: tr("Save pattern")
        fileMode: FileDialog.SaveFile
        nameFilters: [tr("Charto patterns (*.spf)")]
        onAccepted: {
            backend.saveFile(win.localPath(selectedFile))
            if (backend.status === "")
                win.runPending()
            else
                win.pendingAction = null
        }
        onRejected: win.pendingAction = null
    }

    ColorDialog {
        id: colorDialog
        title: tr("Choose a colour")
        onAccepted: backend.addColor(selectedColor.toString())
    }

    FileDialog {
        id: exportPngDialog
        title: tr("Export as PNG image")
        fileMode: FileDialog.SaveFile
        nameFilters: [tr("PNG images (*.png)")]
        onAccepted: win.exportPng(win.localPath(selectedFile))
    }

    FileDialog {
        id: exportSvgDialog
        title: tr("Export as SVG drawing")
        fileMode: FileDialog.SaveFile
        nameFilters: [tr("SVG drawings (*.svg)")]
        onAccepted: backend.exportSvg(win.localPath(selectedFile))
    }

    Dialog {
        id: unsavedDialog
        title: tr("Unsaved changes")
        modal: true
        anchors.centerIn: Overlay.overlay
        standardButtons: Dialog.Save | Dialog.Discard | Dialog.Cancel
        Label { text: tr("The pattern has unsaved changes. Save them first?") }
        onAccepted: {
            if (snap.path !== "") {
                backend.saveFile(snap.path)
                if (backend.status === "")
                    win.runPending()
                else
                    win.pendingAction = null
            } else {
                win.chooseFile("save")
            }
        }
        onDiscarded: win.runPending()
        onRejected: win.pendingAction = null
    }

    Dialog {
        id: recoveryDialog
        property string path: ""
        title: tr("Recovered changes")
        modal: true
        anchors.centerIn: Overlay.overlay
        standardButtons: Dialog.Yes | Dialog.No
        Label {
            width: 380
            wrapMode: Text.Wrap
            text: tr("Charto found unsaved changes to this pattern from an earlier session. Open the recovered version?")
        }
        onAccepted: backend.openRecovery(path)
        onRejected: backend.openFile(path)
    }

    Dialog {
        id: labelDialog
        property string kind: "row"
        property int index: 0
        title: kind === "row" ? tr("Row label") : tr("Stitch label")
        modal: true
        anchors.centerIn: Overlay.overlay
        standardButtons: Dialog.Ok | Dialog.Cancel

        function edit(k, i, text) {
            kind = k
            index = i
            labelField.text = text
            open()
            labelField.forceActiveFocus()
            labelField.selectAll()
        }

        TextField {
            id: labelField
            width: 260
            selectByMouse: true
            onAccepted: labelDialog.accept()
        }
        onAccepted: {
            if (kind === "row")
                backend.setRowLabel(index, labelField.text)
            else
                backend.setColumnLabel(index, labelField.text)
        }
    }

    ColorDialog {
        id: hlColorDialog
        title: tr("Row highlight colour")
        onAccepted: prefDialog.hlColor = selectedColor
    }

    Dialog {
        id: prefDialog
        title: tr("Preferences")
        modal: true
        anchors.centerIn: Overlay.overlay
        standardButtons: Dialog.Ok | Dialog.Cancel
        property color hlColor: "#808080"

        function load() {
            var p = JSON.parse(backend.settingsJson())
            cellW.value = p.cellW
            cellH.value = p.cellH
            rowMode.currentIndex = p.rowMode
            rowStart.value = p.rowStart
            evenLoc.currentIndex = p.evenLoc === "left" ? 0 : 1
            oddLoc.currentIndex = p.oddLoc === "left" ? 0 : 1
            rowInterval.value = p.rowInterval
            rowIntervalStart.value = p.rowIntervalStart
            colMode.currentIndex = p.colMode
            colInterval.value = p.colInterval
            colIntervalStart.value = p.colIntervalStart
            rowsEditable.checked = p.rowsEditable
            colsEditable.checked = p.colsEditable
            hlOn.checked = p.hlOn
            hlOpacity.value = p.hlOpacity
            hlStart.currentIndex = p.hlStart
            hlColor = p.hlColor
            labelFont.text = p.labelFont
            labelSize.value = Math.round(p.labelSize)
            legendFont.text = p.legendFont
            legendSize.value = Math.round(p.legendSize)
            open()
        }

        onAccepted: backend.applySettings(JSON.stringify({
            cellW: cellW.value,
            cellH: cellH.value,
            rowMode: rowMode.currentIndex,
            rowStart: rowStart.value,
            evenLoc: evenLoc.currentIndex === 0 ? "left" : "right",
            oddLoc: oddLoc.currentIndex === 0 ? "left" : "right",
            rowInterval: rowInterval.value,
            rowIntervalStart: rowIntervalStart.value,
            colMode: colMode.currentIndex,
            colInterval: colInterval.value,
            colIntervalStart: colIntervalStart.value,
            rowsEditable: rowsEditable.checked,
            colsEditable: colsEditable.checked,
            hlOn: hlOn.checked,
            hlOpacity: hlOpacity.value,
            hlStart: hlStart.currentIndex,
            hlColor: prefDialog.hlColor.toString(),
            labelFont: labelFont.text,
            labelSize: labelSize.value,
            legendFont: legendFont.text,
            legendSize: legendSize.value
        }))

        ScrollView {
            implicitWidth: 500
            implicitHeight: 520
            clip: true

            GridLayout {
                columns: 2
                columnSpacing: 12
                rowSpacing: 4

                Label { text: tr("Chart"); font.bold: true; Layout.columnSpan: 2 }
                Label { text: tr("Cell width (pixels)") }
                SpinBox { id: cellW; from: 10; to: 200; editable: true }
                Label { text: tr("Cell height (pixels)") }
                SpinBox { id: cellH; from: 10; to: 200; editable: true }

                Label { text: tr("Row labels"); font.bold: true; Layout.columnSpan: 2 }
                Label { text: tr("Show") }
                ComboBox {
                    id: rowMode
                    model: [tr("All rows"), tr("Every n-th row"), tr("Odd rows"), tr("Even rows")]
                }
                Label { text: tr("Number of the bottom row") }
                SpinBox { id: rowStart; from: 0; to: 100000; editable: true }
                Label { text: tr("Interval (n)") }
                SpinBox { id: rowInterval; from: 1; to: 1000; editable: true }
                Label { text: tr("First numbered row of the interval") }
                SpinBox { id: rowIntervalStart; from: 0; to: 100000; editable: true }
                Label { text: tr("Odd rows are labelled on the") }
                ComboBox { id: oddLoc; model: [tr("left"), tr("right")] }
                Label { text: tr("Even rows are labelled on the") }
                ComboBox { id: evenLoc; model: [tr("left"), tr("right")] }
                CheckBox { id: rowsEditable; text: tr("Row labels can be edited (double-click)"); Layout.columnSpan: 2 }

                Label { text: tr("Stitch labels"); font.bold: true; Layout.columnSpan: 2 }
                Label { text: tr("Show") }
                ComboBox { id: colMode; model: [tr("All stitches"), tr("Every n-th stitch")] }
                Label { text: tr("Interval (n)") }
                SpinBox { id: colInterval; from: 1; to: 1000; editable: true }
                Label { text: tr("First numbered stitch of the interval") }
                SpinBox { id: colIntervalStart; from: 0; to: 100000; editable: true }
                CheckBox { id: colsEditable; text: tr("Stitch labels can be edited (double-click)"); Layout.columnSpan: 2 }

                Label { text: tr("Row highlighting"); font.bold: true; Layout.columnSpan: 2 }
                CheckBox { id: hlOn; text: tr("Highlight every other row"); Layout.columnSpan: 2 }
                Label { text: tr("Highlight") }
                ComboBox { id: hlStart; model: [tr("odd rows (1, 3, 5, …)"), tr("even rows (2, 4, 6, …)")] }
                Label { text: tr("Strength (%)") }
                SpinBox { id: hlOpacity; from: 0; to: 100; editable: true }
                Label { text: tr("Colour") }
                Rectangle {
                    implicitWidth: 60
                    implicitHeight: 26
                    radius: 3
                    color: prefDialog.hlColor
                    border.color: "#777"
                    MouseArea { anchors.fill: parent; onClicked: hlColorDialog.open() }
                }

                Label { text: tr("Fonts"); font.bold: true; Layout.columnSpan: 2 }
                Label { text: tr("Label font") }
                TextField { id: labelFont; selectByMouse: true }
                Label { text: tr("Label size") }
                SpinBox { id: labelSize; from: 1; to: 200; editable: true }
                Label { text: tr("Legend font") }
                TextField { id: legendFont; selectByMouse: true }
                Label { text: tr("Legend size") }
                SpinBox { id: legendSize; from: 1; to: 200; editable: true }
            }
        }
    }

    // keep a recovery copy of unsaved work
    Timer {
        interval: 120000
        repeat: true
        running: true
        onTriggered: backend.autosave()
    }

    Dialog {
        id: newDialog
        title: tr("New pattern")
        modal: true
        anchors.centerIn: Overlay.overlay
        standardButtons: Dialog.Ok | Dialog.Cancel
        GridLayout {
            columns: 2
            Label { text: tr("Rows") }
            SpinBox { id: newRows; from: 1; to: 500; value: 10; editable: true }
            Label { text: tr("Stitches") }
            SpinBox { id: newCols; from: 1; to: 500; value: 10; editable: true }
        }
        onAccepted: backend.newChart(newRows.value, newCols.value)
    }

    Dialog {
        id: countDialog
        property string kind: ""
        property string label: ""
        title: label
        modal: true
        anchors.centerIn: Overlay.overlay
        standardButtons: Dialog.Ok | Dialog.Cancel

        function ask(k, text) {
            kind = k
            label = text.replace("…", "")
            countBox.value = 1
            open()
        }

        SpinBox { id: countBox; from: 1; to: 500; value: 1; editable: true }

        onAccepted: {
            var n = countBox.value
            if (kind === "rowsAbove") backend.insertRows(n, true)
            else if (kind === "rowsBelow") backend.insertRows(n, false)
            else if (kind === "colsLeft") backend.insertColumns(n, true)
            else if (kind === "colsRight") backend.insertColumns(n, false)
            else if (kind === "repeat") backend.addRowRepeat(n)
        }
    }

    Menu {
        id: cellMenu
        property int col: 0
        property int row: 0
        MenuItem {
            text: tr("Paste here")
            enabled: snap.hasClipboard
            onTriggered: backend.paste(cellMenu.col, cellMenu.row)
        }
        MenuItem { text: tr("Select row"); onTriggered: backend.selectRow(cellMenu.row) }
        MenuItem { text: tr("Select stitch column"); onTriggered: backend.selectColumn(cellMenu.col) }
        MenuSeparator {}
        MenuItem {
            text: tr("Select all cells with this colour")
            onTriggered: backend.selectSameColor(cellMenu.row, cellMenu.col)
        }
        MenuItem {
            text: tr("Select all cells with this symbol")
            onTriggered: backend.selectSameSymbol(cellMenu.row, cellMenu.col)
        }
    }

    // ---- layout -----------------------------------------------------------

    RowLayout {
        anchors.fill: parent
        spacing: 0

        // left: tools and symbol palette
        Pane {
            Layout.preferredWidth: 310
            Layout.fillHeight: true
            padding: 8

            ColumnLayout {
                anchors.fill: parent
                spacing: 6

                Label { text: tr("Mode"); font.bold: true }
                RowLayout {
                    RadioButton { text: tr("Select"); checked: snap.mode === 0; onClicked: backend.setMode(0) }
                    RadioButton { text: tr("Hide"); checked: snap.mode === 1; onClicked: backend.setMode(1) }
                    RadioButton { text: tr("Unhide"); checked: snap.mode === 2; onClicked: backend.setMode(2) }
                }

                Label { text: tr("Colour"); font.bold: true }
                Flow {
                    Layout.fillWidth: true
                    spacing: 4
                    Repeater {
                        model: snap.colors
                        delegate: Rectangle {
                            required property var modelData
                            width: 26
                            height: 26
                            radius: 3
                            color: modelData
                            border.width: modelData === snap.color ? 3 : 1
                            border.color: modelData === snap.color ? "#3366cc" : "#777"
                            MouseArea { anchors.fill: parent; onClicked: backend.setColor(modelData) }
                        }
                    }
                    Button { text: "+"; width: 26; height: 26; onClicked: colorDialog.open() }
                }
                Button {
                    text: tr("Apply colour to selection")
                    enabled: snap.selected > 0
                    onClicked: backend.applyColor()
                }

                Label { visible: snap.recentSymbols.length > 0; text: tr("Recently used"); font.bold: true }
                Flow {
                    visible: snap.recentSymbols.length > 0
                    Layout.fillWidth: true
                    spacing: 4
                    Repeater {
                        model: snap.recentSymbols
                        delegate: Rectangle {
                            id: recentSwatch
                            required property var modelData
                            width: Math.min(30 * modelData.w, 150)
                            height: 30
                            color: modelData.bg !== "" ? modelData.bg : "white"
                            border.width: modelData.name === snap.active ? 3 : 1
                            border.color: modelData.name === snap.active ? "#3366cc" : "#999"
                            Image {
                                anchors.fill: parent
                                anchors.margins: 2
                                source: recentSwatch.modelData.svg
                                sourceSize: Qt.size(width * 2, height * 2)
                                fillMode: Image.PreserveAspectFit
                            }
                            ToolTip.visible: recentHover.hovered
                            ToolTip.text: tr(recentSwatch.modelData.name) + " – " + tr(recentSwatch.modelData.description)
                            HoverHandler { id: recentHover }
                            MouseArea {
                                anchors.fill: parent
                                onClicked: backend.setActiveSymbol(
                                    snap.active === recentSwatch.modelData.name ? "" : recentSwatch.modelData.name)
                            }
                        }
                    }
                }

                Label {
                    text: snap.active === "" ? tr("Symbols (none active)") : tr("Symbols – active: %1").arg(tr(snap.active))
                    font.bold: true
                }

                ListView {
                    Layout.fillWidth: true
                    Layout.fillHeight: true
                    clip: true
                    spacing: 8
                    model: win.symbolPalette
                    ScrollBar.vertical: ScrollBar {}
                    delegate: Column {
                        required property var modelData
                        width: ListView.view.width - 12
                        spacing: 3
                        Label { text: tr(modelData.category); font.italic: true }
                        Flow {
                            width: parent.width
                            spacing: 4
                            Repeater {
                                model: modelData.symbols
                                delegate: Rectangle {
                                    id: swatch
                                    required property var modelData
                                    width: Math.min(38 * modelData.w, 190)
                                    height: 38
                                    color: modelData.bg !== "" ? modelData.bg : "white"
                                    border.width: modelData.name === snap.active ? 3 : 1
                                    border.color: modelData.name === snap.active ? "#3366cc" : "#999"
                                    Image {
                                        anchors.fill: parent
                                        anchors.margins: 2
                                        source: swatch.modelData.svg
                                        sourceSize: Qt.size(width * 2, height * 2)
                                        fillMode: Image.PreserveAspectFit
                                    }
                                    ToolTip.visible: hover.hovered
                                    ToolTip.text: tr(swatch.modelData.name) + " – " + tr(swatch.modelData.description)
                                    HoverHandler { id: hover }
                                    MouseArea {
                                        anchors.fill: parent
                                        onClicked: backend.setActiveSymbol(
                                            snap.active === swatch.modelData.name ? "" : swatch.modelData.name)
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // right: the chart
        Rectangle {
            Layout.fillWidth: true
            Layout.fillHeight: true
            color: "#e6e6e6"
            clip: true

            Flickable {
                id: flick
                anchors.fill: parent
                contentWidth: Math.max(width, canvas.width)
                contentHeight: Math.max(height, canvas.height)
                boundsBehavior: Flickable.StopAtBounds
                ScrollBar.vertical: ScrollBar {}
                ScrollBar.horizontal: ScrollBar {}

                WheelHandler {
                    acceptedModifiers: Qt.ControlModifier
                    onWheel: (event) => win.zoomBy(event.angleDelta.y > 0 ? 1.1 : 1 / 1.1)
                }

                Item {
                    id: canvas
                    width: win.leftMargin + snap.cols * win.cw + win.rightMargin
                    height: snap.rows * win.ch + win.bottomMargin

                    // cells
                    Repeater {
                        model: snap.cells
                        delegate: Rectangle {
                            id: cellItem
                            required property var modelData
                            x: win.leftMargin + modelData.c * win.cw
                            y: modelData.r * win.ch
                            width: modelData.w * win.cw
                            height: win.ch
                            color: modelData.hid ? ((snap.mode !== 0 && !win.printing) ? "#18000000" : "transparent") : modelData.bg
                            border.width: 1
                            border.color: (modelData.hid && (snap.mode === 0 || win.printing)) ? "transparent" : "#8a8a8a"

                            Image {
                                anchors.fill: parent
                                anchors.margins: 1
                                visible: !cellItem.modelData.hid && cellItem.modelData.svg !== ""
                                source: cellItem.modelData.svg
                                sourceSize: Qt.size(width * 2 * win.printScale, height * 2 * win.printScale)
                                fillMode: Image.PreserveAspectFit
                            }
                            Text {
                                anchors.centerIn: parent
                                visible: !cellItem.modelData.hid && cellItem.modelData.svg === ""
                                text: "?"
                                color: "#a00"
                            }
                            Rectangle {
                                anchors.fill: parent
                                visible: cellItem.modelData.sel && !win.printing
                                color: "#553399ff"
                                border.width: 2
                                border.color: "#2255bb"
                            }
                        }
                    }

                    // row highlighting
                    Repeater {
                        model: snap.highlight.on ? snap.highlight.rows : []
                        delegate: Rectangle {
                            required property var modelData
                            x: win.leftMargin
                            y: modelData * win.ch
                            width: snap.cols * win.cw
                            height: win.ch
                            color: snap.highlight.color
                            opacity: snap.highlight.opacity
                        }
                    }

                    // row repeats: a bracket on the left with the multiplier
                    Repeater {
                        model: snap.repeats
                        delegate: Item {
                            required property var modelData
                            x: 0
                            y: modelData.first * win.ch
                            width: win.leftMargin
                            height: (modelData.last - modelData.first + 1) * win.ch
                            Rectangle { x: 30; width: 3; height: parent.height; color: "#cc3300" }
                            Text {
                                x: 0
                                width: 28
                                height: parent.height
                                horizontalAlignment: Text.AlignRight
                                verticalAlignment: Text.AlignVCenter
                                text: "×" + modelData.mult
                                color: "#cc3300"
                                font.bold: true
                            }
                        }
                    }

                    // row labels (click selects the row)
                    Repeater {
                        model: snap.rowLabels
                        delegate: Text {
                            required property var modelData
                            readonly property bool onLeft: modelData.side === "left"
                            x: onLeft ? 36 : win.leftMargin + snap.cols * win.cw + 6
                            y: modelData.row * win.ch
                            width: onLeft ? win.leftMargin - 42 : win.rightMargin - 10
                            height: win.ch
                            horizontalAlignment: onLeft ? Text.AlignRight : Text.AlignLeft
                            verticalAlignment: Text.AlignVCenter
                            text: modelData.text
                            color: "#333"
                            font.pixelSize: Math.max(9, win.ch * 0.4)
                            MouseArea {
                                anchors.fill: parent
                                onClicked: backend.selectRow(modelData.row)
                                onDoubleClicked: if (snap.rowsEditable) labelDialog.edit("row", modelData.row, modelData.text)
                            }
                        }
                    }

                    // column labels (click selects the column)
                    Repeater {
                        model: snap.colLabels
                        delegate: Text {
                            required property var modelData
                            x: win.leftMargin + modelData.col * win.cw
                            y: snap.rows * win.ch + 4
                            width: win.cw
                            height: 22
                            horizontalAlignment: Text.AlignHCenter
                            verticalAlignment: Text.AlignVCenter
                            text: modelData.text
                            color: "#333"
                            font.pixelSize: Math.max(9, win.ch * 0.4)
                            MouseArea {
                                anchors.fill: parent
                                onClicked: backend.selectColumn(modelData.col)
                                onDoubleClicked: if (snap.colsEditable) labelDialog.edit("col", modelData.col, modelData.text)
                            }
                        }
                    }

                    // rubber band
                    Rectangle {
                        visible: gridArea.dragging && gridArea.pressed
                        x: gridArea.x + Math.min(gridArea.p0.x, gridArea.mouseX)
                        y: gridArea.y + Math.min(gridArea.p0.y, gridArea.mouseY)
                        width: Math.abs(gridArea.mouseX - gridArea.p0.x)
                        height: Math.abs(gridArea.mouseY - gridArea.p0.y)
                        color: "#223399ff"
                        border.color: "#2255bb"
                        border.width: 1
                    }

                    // clicks and drags on the grid
                    MouseArea {
                        id: gridArea
                        x: win.leftMargin
                        y: 0
                        width: snap.cols * win.cw
                        height: snap.rows * win.ch
                        acceptedButtons: Qt.LeftButton | Qt.RightButton
                        property point p0: Qt.point(0, 0)
                        property bool dragging: false

                        function colAt(px) { return Math.max(0, Math.min(snap.cols - 1, Math.floor(px / win.cw))) }
                        function rowAt(py) { return Math.max(0, Math.min(snap.rows - 1, Math.floor(py / win.ch))) }

                        onPressed: (mouse) => {
                            p0 = Qt.point(mouse.x, mouse.y)
                            dragging = false
                        }
                        onPositionChanged: (mouse) => {
                            if ((mouse.buttons & Qt.LeftButton)
                                    && Math.abs(mouse.x - p0.x) + Math.abs(mouse.y - p0.y) > 6)
                                dragging = true
                        }
                        onReleased: (mouse) => {
                            if (mouse.button === Qt.RightButton) {
                                cellMenu.col = colAt(mouse.x)
                                cellMenu.row = rowAt(mouse.y)
                                cellMenu.popup()
                            } else if (dragging) {
                                var c0 = colAt(Math.min(p0.x, mouse.x))
                                var c1 = colAt(Math.max(p0.x, mouse.x))
                                var r0 = rowAt(Math.min(p0.y, mouse.y))
                                var r1 = rowAt(Math.max(p0.y, mouse.y))
                                backend.selectRect(c0, r0, c1 - c0 + 1, r1 - r0 + 1)
                            } else {
                                backend.clickCell(rowAt(mouse.y), colAt(mouse.x))
                            }
                            dragging = false
                        }
                    }
                }
            }
        }
    }

    // ---- status bar -------------------------------------------------------

    footer: ToolBar {
        RowLayout {
            anchors.fill: parent
            anchors.leftMargin: 8
            anchors.rightMargin: 8
            Label {
                text: tr("%1 × %2 cells   selected: %3").arg(snap.rows).arg(snap.cols).arg(snap.selected)
                      + "   " + Math.round(win.zoom * 100) + "%"
            }
            Label {
                Layout.fillWidth: true
                horizontalAlignment: Text.AlignRight
                text: backend.status
                color: "#b05000"
                elide: Text.ElideRight
            }
        }
    }
}
