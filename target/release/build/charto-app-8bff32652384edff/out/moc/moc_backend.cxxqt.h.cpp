/****************************************************************************
** Meta object code from reading C++ file 'backend.cxxqt.h'
**
** Created by: The Qt Meta Object Compiler version 69 (Qt 6.10.2)
**
** WARNING! All changes made in this file will be lost!
*****************************************************************************/

#include "../cxxqtbuild/include/charto-app/src/backend.cxxqt.h"
#include <QtCore/qmetatype.h>

#include <QtCore/qtmochelpers.h>

#include <memory>


#include <QtCore/qxptype_traits.h>
#if !defined(Q_MOC_OUTPUT_REVISION)
#error "The header file 'backend.cxxqt.h' doesn't include <QObject>."
#elif Q_MOC_OUTPUT_REVISION != 69
#error "This file was generated using the moc from 6.10.2. It"
#error "cannot be used with the include files from this version of Qt."
#error "(The moc has changed too much.)"
#endif

#ifndef Q_CONSTINIT
#define Q_CONSTINIT
#endif

QT_WARNING_PUSH
QT_WARNING_DISABLE_DEPRECATED
QT_WARNING_DISABLE_GCC("-Wuseless-cast")
namespace {
struct qt_meta_tag_ZN12ChartBackendE_t {};
} // unnamed namespace

template <> constexpr inline auto ChartBackend::qt_create_metaobjectdata<qt_meta_tag_ZN12ChartBackendE_t>()
{
    namespace QMC = QtMocConstants;
    QtMocHelpers::StringRefStorage qt_stringData {
        "ChartBackend",
        "QML.Element",
        "snapshotChanged",
        "",
        "statusChanged",
        "pickedChanged",
        "setSnapshot",
        "value",
        "setStatus",
        "setPicked",
        "paletteJson",
        "newChart",
        "::std::int32_t",
        "rows",
        "cols",
        "openFile",
        "path",
        "saveFile",
        "setActiveSymbol",
        "name",
        "setColor",
        "hex",
        "addColor",
        "setMode",
        "mode",
        "clickCell",
        "row",
        "col",
        "selectRect",
        "num_cols",
        "num_rows",
        "selectRow",
        "selectColumn",
        "clearSelection",
        "applyColor",
        "undo",
        "redo",
        "copy",
        "paste",
        "insertRows",
        "num",
        "above",
        "deleteRows",
        "insertColumns",
        "left",
        "deleteColumns",
        "addRowRepeat",
        "multiplicity",
        "deleteRowRepeat",
        "hasRecovery",
        "openRecovery",
        "autosave",
        "endSession",
        "clearRecentFiles",
        "selectSameColor",
        "selectSameSymbol",
        "setRowLabel",
        "text",
        "setColumnLabel",
        "settingsJson",
        "applySettings",
        "json",
        "exportSvg",
        "printTempPath",
        "translate",
        "startupFile",
        "printImage",
        "png_path",
        "title",
        "chooseFile",
        "purpose",
        "current",
        "reportStatus",
        "message",
        "snapshot",
        "status",
        "picked"
    };

    QtMocHelpers::UintData qt_methods {
        // Signal 'snapshotChanged'
        QtMocHelpers::SignalData<void()>(2, 3, QMC::AccessPublic, QMetaType::Void),
        // Signal 'statusChanged'
        QtMocHelpers::SignalData<void()>(4, 3, QMC::AccessPublic, QMetaType::Void),
        // Signal 'pickedChanged'
        QtMocHelpers::SignalData<void()>(5, 3, QMC::AccessPublic, QMetaType::Void),
        // Slot 'setSnapshot'
        QtMocHelpers::SlotData<void(QString)>(6, 3, QMC::AccessPublic, QMetaType::Void, {{
            { QMetaType::QString, 7 },
        }}),
        // Slot 'setStatus'
        QtMocHelpers::SlotData<void(QString)>(8, 3, QMC::AccessPublic, QMetaType::Void, {{
            { QMetaType::QString, 7 },
        }}),
        // Slot 'setPicked'
        QtMocHelpers::SlotData<void(QString)>(9, 3, QMC::AccessPublic, QMetaType::Void, {{
            { QMetaType::QString, 7 },
        }}),
        // Method 'paletteJson'
        QtMocHelpers::MethodData<QString() const>(10, 3, QMC::AccessPublic, QMetaType::QString),
        // Method 'newChart'
        QtMocHelpers::MethodData<void(::std::int32_t, ::std::int32_t)>(11, 3, QMC::AccessPublic, QMetaType::Void, {{
            { 0x80000000 | 12, 13 }, { 0x80000000 | 12, 14 },
        }}),
        // Method 'openFile'
        QtMocHelpers::MethodData<void(QString const &)>(15, 3, QMC::AccessPublic, QMetaType::Void, {{
            { QMetaType::QString, 16 },
        }}),
        // Method 'saveFile'
        QtMocHelpers::MethodData<void(QString const &)>(17, 3, QMC::AccessPublic, QMetaType::Void, {{
            { QMetaType::QString, 16 },
        }}),
        // Method 'setActiveSymbol'
        QtMocHelpers::MethodData<void(QString const &)>(18, 3, QMC::AccessPublic, QMetaType::Void, {{
            { QMetaType::QString, 19 },
        }}),
        // Method 'setColor'
        QtMocHelpers::MethodData<void(QString const &)>(20, 3, QMC::AccessPublic, QMetaType::Void, {{
            { QMetaType::QString, 21 },
        }}),
        // Method 'addColor'
        QtMocHelpers::MethodData<void(QString const &)>(22, 3, QMC::AccessPublic, QMetaType::Void, {{
            { QMetaType::QString, 21 },
        }}),
        // Method 'setMode'
        QtMocHelpers::MethodData<void(::std::int32_t)>(23, 3, QMC::AccessPublic, QMetaType::Void, {{
            { 0x80000000 | 12, 24 },
        }}),
        // Method 'clickCell'
        QtMocHelpers::MethodData<void(::std::int32_t, ::std::int32_t)>(25, 3, QMC::AccessPublic, QMetaType::Void, {{
            { 0x80000000 | 12, 26 }, { 0x80000000 | 12, 27 },
        }}),
        // Method 'selectRect'
        QtMocHelpers::MethodData<void(::std::int32_t, ::std::int32_t, ::std::int32_t, ::std::int32_t)>(28, 3, QMC::AccessPublic, QMetaType::Void, {{
            { 0x80000000 | 12, 27 }, { 0x80000000 | 12, 26 }, { 0x80000000 | 12, 29 }, { 0x80000000 | 12, 30 },
        }}),
        // Method 'selectRow'
        QtMocHelpers::MethodData<void(::std::int32_t)>(31, 3, QMC::AccessPublic, QMetaType::Void, {{
            { 0x80000000 | 12, 26 },
        }}),
        // Method 'selectColumn'
        QtMocHelpers::MethodData<void(::std::int32_t)>(32, 3, QMC::AccessPublic, QMetaType::Void, {{
            { 0x80000000 | 12, 27 },
        }}),
        // Method 'clearSelection'
        QtMocHelpers::MethodData<void()>(33, 3, QMC::AccessPublic, QMetaType::Void),
        // Method 'applyColor'
        QtMocHelpers::MethodData<void()>(34, 3, QMC::AccessPublic, QMetaType::Void),
        // Method 'undo'
        QtMocHelpers::MethodData<void()>(35, 3, QMC::AccessPublic, QMetaType::Void),
        // Method 'redo'
        QtMocHelpers::MethodData<void()>(36, 3, QMC::AccessPublic, QMetaType::Void),
        // Method 'copy'
        QtMocHelpers::MethodData<void()>(37, 3, QMC::AccessPublic, QMetaType::Void),
        // Method 'paste'
        QtMocHelpers::MethodData<void(::std::int32_t, ::std::int32_t)>(38, 3, QMC::AccessPublic, QMetaType::Void, {{
            { 0x80000000 | 12, 27 }, { 0x80000000 | 12, 26 },
        }}),
        // Method 'insertRows'
        QtMocHelpers::MethodData<void(::std::int32_t, bool)>(39, 3, QMC::AccessPublic, QMetaType::Void, {{
            { 0x80000000 | 12, 40 }, { QMetaType::Bool, 41 },
        }}),
        // Method 'deleteRows'
        QtMocHelpers::MethodData<void()>(42, 3, QMC::AccessPublic, QMetaType::Void),
        // Method 'insertColumns'
        QtMocHelpers::MethodData<void(::std::int32_t, bool)>(43, 3, QMC::AccessPublic, QMetaType::Void, {{
            { 0x80000000 | 12, 40 }, { QMetaType::Bool, 44 },
        }}),
        // Method 'deleteColumns'
        QtMocHelpers::MethodData<void()>(45, 3, QMC::AccessPublic, QMetaType::Void),
        // Method 'addRowRepeat'
        QtMocHelpers::MethodData<void(::std::int32_t)>(46, 3, QMC::AccessPublic, QMetaType::Void, {{
            { 0x80000000 | 12, 47 },
        }}),
        // Method 'deleteRowRepeat'
        QtMocHelpers::MethodData<void()>(48, 3, QMC::AccessPublic, QMetaType::Void),
        // Method 'hasRecovery'
        QtMocHelpers::MethodData<bool(QString const &) const>(49, 3, QMC::AccessPublic, QMetaType::Bool, {{
            { QMetaType::QString, 16 },
        }}),
        // Method 'openRecovery'
        QtMocHelpers::MethodData<void(QString const &)>(50, 3, QMC::AccessPublic, QMetaType::Void, {{
            { QMetaType::QString, 16 },
        }}),
        // Method 'autosave'
        QtMocHelpers::MethodData<void()>(51, 3, QMC::AccessPublic, QMetaType::Void),
        // Method 'endSession'
        QtMocHelpers::MethodData<void()>(52, 3, QMC::AccessPublic, QMetaType::Void),
        // Method 'clearRecentFiles'
        QtMocHelpers::MethodData<void()>(53, 3, QMC::AccessPublic, QMetaType::Void),
        // Method 'selectSameColor'
        QtMocHelpers::MethodData<void(::std::int32_t, ::std::int32_t)>(54, 3, QMC::AccessPublic, QMetaType::Void, {{
            { 0x80000000 | 12, 26 }, { 0x80000000 | 12, 27 },
        }}),
        // Method 'selectSameSymbol'
        QtMocHelpers::MethodData<void(::std::int32_t, ::std::int32_t)>(55, 3, QMC::AccessPublic, QMetaType::Void, {{
            { 0x80000000 | 12, 26 }, { 0x80000000 | 12, 27 },
        }}),
        // Method 'setRowLabel'
        QtMocHelpers::MethodData<void(::std::int32_t, QString const &)>(56, 3, QMC::AccessPublic, QMetaType::Void, {{
            { 0x80000000 | 12, 26 }, { QMetaType::QString, 57 },
        }}),
        // Method 'setColumnLabel'
        QtMocHelpers::MethodData<void(::std::int32_t, QString const &)>(58, 3, QMC::AccessPublic, QMetaType::Void, {{
            { 0x80000000 | 12, 27 }, { QMetaType::QString, 57 },
        }}),
        // Method 'settingsJson'
        QtMocHelpers::MethodData<QString() const>(59, 3, QMC::AccessPublic, QMetaType::QString),
        // Method 'applySettings'
        QtMocHelpers::MethodData<void(QString const &)>(60, 3, QMC::AccessPublic, QMetaType::Void, {{
            { QMetaType::QString, 61 },
        }}),
        // Method 'exportSvg'
        QtMocHelpers::MethodData<void(QString const &)>(62, 3, QMC::AccessPublic, QMetaType::Void, {{
            { QMetaType::QString, 16 },
        }}),
        // Method 'printTempPath'
        QtMocHelpers::MethodData<QString() const>(63, 3, QMC::AccessPublic, QMetaType::QString),
        // Method 'translate'
        QtMocHelpers::MethodData<QString(QString const &) const>(64, 3, QMC::AccessPublic, QMetaType::QString, {{
            { QMetaType::QString, 57 },
        }}),
        // Method 'startupFile'
        QtMocHelpers::MethodData<QString() const>(65, 3, QMC::AccessPublic, QMetaType::QString),
        // Method 'printImage'
        QtMocHelpers::MethodData<void(QString const &, QString const &)>(66, 3, QMC::AccessPublic, QMetaType::Void, {{
            { QMetaType::QString, 67 }, { QMetaType::QString, 68 },
        }}),
        // Method 'chooseFile'
        QtMocHelpers::MethodData<void(QString const &, QString const &)>(69, 3, QMC::AccessPublic, QMetaType::Void, {{
            { QMetaType::QString, 70 }, { QMetaType::QString, 71 },
        }}),
        // Method 'reportStatus'
        QtMocHelpers::MethodData<void(QString const &)>(72, 3, QMC::AccessPublic, QMetaType::Void, {{
            { QMetaType::QString, 73 },
        }}),
    };
    QtMocHelpers::UintData qt_properties {
        // property 'snapshot'
        QtMocHelpers::PropertyData<QString>(74, QMetaType::QString, QMC::DefaultPropertyFlags | QMC::Writable | QMC::StdCppSet, 0),
        // property 'status'
        QtMocHelpers::PropertyData<QString>(75, QMetaType::QString, QMC::DefaultPropertyFlags | QMC::Writable | QMC::StdCppSet, 1),
        // property 'picked'
        QtMocHelpers::PropertyData<QString>(76, QMetaType::QString, QMC::DefaultPropertyFlags | QMC::Writable | QMC::StdCppSet, 2),
    };
    QtMocHelpers::UintData qt_enums {
    };
    QtMocHelpers::UintData qt_constructors {};
    QtMocHelpers::ClassInfos qt_classinfo({
            {    1,    0 },
    });
    return QtMocHelpers::metaObjectData<ChartBackend, void>(QMC::MetaObjectFlag{}, qt_stringData,
            qt_methods, qt_properties, qt_enums, qt_constructors, qt_classinfo);
}
Q_CONSTINIT const QMetaObject ChartBackend::staticMetaObject = { {
    QMetaObject::SuperData::link<QObject::staticMetaObject>(),
    qt_staticMetaObjectStaticContent<qt_meta_tag_ZN12ChartBackendE_t>.stringdata,
    qt_staticMetaObjectStaticContent<qt_meta_tag_ZN12ChartBackendE_t>.data,
    qt_static_metacall,
    nullptr,
    qt_staticMetaObjectRelocatingContent<qt_meta_tag_ZN12ChartBackendE_t>.metaTypes,
    nullptr
} };

void ChartBackend::qt_static_metacall(QObject *_o, QMetaObject::Call _c, int _id, void **_a)
{
    auto *_t = static_cast<ChartBackend *>(_o);
    if (_c == QMetaObject::InvokeMetaMethod) {
        switch (_id) {
        case 0: _t->snapshotChanged(); break;
        case 1: _t->statusChanged(); break;
        case 2: _t->pickedChanged(); break;
        case 3: _t->setSnapshot((*reinterpret_cast<std::add_pointer_t<QString>>(_a[1]))); break;
        case 4: _t->setStatus((*reinterpret_cast<std::add_pointer_t<QString>>(_a[1]))); break;
        case 5: _t->setPicked((*reinterpret_cast<std::add_pointer_t<QString>>(_a[1]))); break;
        case 6: { QString _r = _t->paletteJson();
            if (_a[0]) *reinterpret_cast<QString*>(_a[0]) = std::move(_r); }  break;
        case 7: _t->newChart((*reinterpret_cast<std::add_pointer_t<::std::int32_t>>(_a[1])),(*reinterpret_cast<std::add_pointer_t<::std::int32_t>>(_a[2]))); break;
        case 8: _t->openFile((*reinterpret_cast<std::add_pointer_t<QString>>(_a[1]))); break;
        case 9: _t->saveFile((*reinterpret_cast<std::add_pointer_t<QString>>(_a[1]))); break;
        case 10: _t->setActiveSymbol((*reinterpret_cast<std::add_pointer_t<QString>>(_a[1]))); break;
        case 11: _t->setColor((*reinterpret_cast<std::add_pointer_t<QString>>(_a[1]))); break;
        case 12: _t->addColor((*reinterpret_cast<std::add_pointer_t<QString>>(_a[1]))); break;
        case 13: _t->setMode((*reinterpret_cast<std::add_pointer_t<::std::int32_t>>(_a[1]))); break;
        case 14: _t->clickCell((*reinterpret_cast<std::add_pointer_t<::std::int32_t>>(_a[1])),(*reinterpret_cast<std::add_pointer_t<::std::int32_t>>(_a[2]))); break;
        case 15: _t->selectRect((*reinterpret_cast<std::add_pointer_t<::std::int32_t>>(_a[1])),(*reinterpret_cast<std::add_pointer_t<::std::int32_t>>(_a[2])),(*reinterpret_cast<std::add_pointer_t<::std::int32_t>>(_a[3])),(*reinterpret_cast<std::add_pointer_t<::std::int32_t>>(_a[4]))); break;
        case 16: _t->selectRow((*reinterpret_cast<std::add_pointer_t<::std::int32_t>>(_a[1]))); break;
        case 17: _t->selectColumn((*reinterpret_cast<std::add_pointer_t<::std::int32_t>>(_a[1]))); break;
        case 18: _t->clearSelection(); break;
        case 19: _t->applyColor(); break;
        case 20: _t->undo(); break;
        case 21: _t->redo(); break;
        case 22: _t->copy(); break;
        case 23: _t->paste((*reinterpret_cast<std::add_pointer_t<::std::int32_t>>(_a[1])),(*reinterpret_cast<std::add_pointer_t<::std::int32_t>>(_a[2]))); break;
        case 24: _t->insertRows((*reinterpret_cast<std::add_pointer_t<::std::int32_t>>(_a[1])),(*reinterpret_cast<std::add_pointer_t<bool>>(_a[2]))); break;
        case 25: _t->deleteRows(); break;
        case 26: _t->insertColumns((*reinterpret_cast<std::add_pointer_t<::std::int32_t>>(_a[1])),(*reinterpret_cast<std::add_pointer_t<bool>>(_a[2]))); break;
        case 27: _t->deleteColumns(); break;
        case 28: _t->addRowRepeat((*reinterpret_cast<std::add_pointer_t<::std::int32_t>>(_a[1]))); break;
        case 29: _t->deleteRowRepeat(); break;
        case 30: { bool _r = _t->hasRecovery((*reinterpret_cast<std::add_pointer_t<QString>>(_a[1])));
            if (_a[0]) *reinterpret_cast<bool*>(_a[0]) = std::move(_r); }  break;
        case 31: _t->openRecovery((*reinterpret_cast<std::add_pointer_t<QString>>(_a[1]))); break;
        case 32: _t->autosave(); break;
        case 33: _t->endSession(); break;
        case 34: _t->clearRecentFiles(); break;
        case 35: _t->selectSameColor((*reinterpret_cast<std::add_pointer_t<::std::int32_t>>(_a[1])),(*reinterpret_cast<std::add_pointer_t<::std::int32_t>>(_a[2]))); break;
        case 36: _t->selectSameSymbol((*reinterpret_cast<std::add_pointer_t<::std::int32_t>>(_a[1])),(*reinterpret_cast<std::add_pointer_t<::std::int32_t>>(_a[2]))); break;
        case 37: _t->setRowLabel((*reinterpret_cast<std::add_pointer_t<::std::int32_t>>(_a[1])),(*reinterpret_cast<std::add_pointer_t<QString>>(_a[2]))); break;
        case 38: _t->setColumnLabel((*reinterpret_cast<std::add_pointer_t<::std::int32_t>>(_a[1])),(*reinterpret_cast<std::add_pointer_t<QString>>(_a[2]))); break;
        case 39: { QString _r = _t->settingsJson();
            if (_a[0]) *reinterpret_cast<QString*>(_a[0]) = std::move(_r); }  break;
        case 40: _t->applySettings((*reinterpret_cast<std::add_pointer_t<QString>>(_a[1]))); break;
        case 41: _t->exportSvg((*reinterpret_cast<std::add_pointer_t<QString>>(_a[1]))); break;
        case 42: { QString _r = _t->printTempPath();
            if (_a[0]) *reinterpret_cast<QString*>(_a[0]) = std::move(_r); }  break;
        case 43: { QString _r = _t->translate((*reinterpret_cast<std::add_pointer_t<QString>>(_a[1])));
            if (_a[0]) *reinterpret_cast<QString*>(_a[0]) = std::move(_r); }  break;
        case 44: { QString _r = _t->startupFile();
            if (_a[0]) *reinterpret_cast<QString*>(_a[0]) = std::move(_r); }  break;
        case 45: _t->printImage((*reinterpret_cast<std::add_pointer_t<QString>>(_a[1])),(*reinterpret_cast<std::add_pointer_t<QString>>(_a[2]))); break;
        case 46: _t->chooseFile((*reinterpret_cast<std::add_pointer_t<QString>>(_a[1])),(*reinterpret_cast<std::add_pointer_t<QString>>(_a[2]))); break;
        case 47: _t->reportStatus((*reinterpret_cast<std::add_pointer_t<QString>>(_a[1]))); break;
        default: ;
        }
    }
    if (_c == QMetaObject::IndexOfMethod) {
        if (QtMocHelpers::indexOfMethod<void (ChartBackend::*)()>(_a, &ChartBackend::snapshotChanged, 0))
            return;
        if (QtMocHelpers::indexOfMethod<void (ChartBackend::*)()>(_a, &ChartBackend::statusChanged, 1))
            return;
        if (QtMocHelpers::indexOfMethod<void (ChartBackend::*)()>(_a, &ChartBackend::pickedChanged, 2))
            return;
    }
    if (_c == QMetaObject::ReadProperty) {
        void *_v = _a[0];
        switch (_id) {
        case 0: *reinterpret_cast<QString*>(_v) = _t->getSnapshot(); break;
        case 1: *reinterpret_cast<QString*>(_v) = _t->getStatus(); break;
        case 2: *reinterpret_cast<QString*>(_v) = _t->getPicked(); break;
        default: break;
        }
    }
    if (_c == QMetaObject::WriteProperty) {
        void *_v = _a[0];
        switch (_id) {
        case 0: _t->setSnapshot(*reinterpret_cast<QString*>(_v)); break;
        case 1: _t->setStatus(*reinterpret_cast<QString*>(_v)); break;
        case 2: _t->setPicked(*reinterpret_cast<QString*>(_v)); break;
        default: break;
        }
    }
}

const QMetaObject *ChartBackend::metaObject() const
{
    return QObject::d_ptr->metaObject ? QObject::d_ptr->dynamicMetaObject() : &staticMetaObject;
}

void *ChartBackend::qt_metacast(const char *_clname)
{
    if (!_clname) return nullptr;
    if (!strcmp(_clname, qt_staticMetaObjectStaticContent<qt_meta_tag_ZN12ChartBackendE_t>.strings))
        return static_cast<void*>(this);
    return QObject::qt_metacast(_clname);
}

int ChartBackend::qt_metacall(QMetaObject::Call _c, int _id, void **_a)
{
    _id = QObject::qt_metacall(_c, _id, _a);
    if (_id < 0)
        return _id;
    if (_c == QMetaObject::InvokeMetaMethod) {
        if (_id < 48)
            qt_static_metacall(this, _c, _id, _a);
        _id -= 48;
    }
    if (_c == QMetaObject::RegisterMethodArgumentMetaType) {
        if (_id < 48)
            *reinterpret_cast<QMetaType *>(_a[0]) = QMetaType();
        _id -= 48;
    }
    if (_c == QMetaObject::ReadProperty || _c == QMetaObject::WriteProperty
            || _c == QMetaObject::ResetProperty || _c == QMetaObject::BindableProperty
            || _c == QMetaObject::RegisterPropertyMetaType) {
        qt_static_metacall(this, _c, _id, _a);
        _id -= 3;
    }
    return _id;
}

// SIGNAL 0
void ChartBackend::snapshotChanged()
{
    QMetaObject::activate(this, &staticMetaObject, 0, nullptr);
}

// SIGNAL 1
void ChartBackend::statusChanged()
{
    QMetaObject::activate(this, &staticMetaObject, 1, nullptr);
}

// SIGNAL 2
void ChartBackend::pickedChanged()
{
    QMetaObject::activate(this, &staticMetaObject, 2, nullptr);
}
QT_WARNING_POP
