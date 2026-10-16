/****************************************************************************
** Meta object code from reading C++ file 'org_charto_app_plugin.cpp'
**
** Created by: The Qt Meta Object Compiler version 69 (Qt 6.10.2)
**
** WARNING! All changes made in this file will be lost!
*****************************************************************************/

#include <QtCore/qmetatype.h>
#include <QtCore/qplugin.h>

#include <QtCore/qtmochelpers.h>

#include <memory>


#include <QtCore/qxptype_traits.h>
#if !defined(Q_MOC_OUTPUT_REVISION)
#error "The header file 'org_charto_app_plugin.cpp' doesn't include <QObject>."
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
struct qt_meta_tag_ZN21org_charto_app_pluginE_t {};
} // unnamed namespace

template <> constexpr inline auto org_charto_app_plugin::qt_create_metaobjectdata<qt_meta_tag_ZN21org_charto_app_pluginE_t>()
{
    namespace QMC = QtMocConstants;
    QtMocHelpers::StringRefStorage qt_stringData {
        "org_charto_app_plugin"
    };

    QtMocHelpers::UintData qt_methods {
    };
    QtMocHelpers::UintData qt_properties {
    };
    QtMocHelpers::UintData qt_enums {
    };
    return QtMocHelpers::metaObjectData<org_charto_app_plugin, qt_meta_tag_ZN21org_charto_app_pluginE_t>(QMC::MetaObjectFlag{}, qt_stringData,
            qt_methods, qt_properties, qt_enums);
}
Q_CONSTINIT const QMetaObject org_charto_app_plugin::staticMetaObject = { {
    QMetaObject::SuperData::link<QQmlEngineExtensionPlugin::staticMetaObject>(),
    qt_staticMetaObjectStaticContent<qt_meta_tag_ZN21org_charto_app_pluginE_t>.stringdata,
    qt_staticMetaObjectStaticContent<qt_meta_tag_ZN21org_charto_app_pluginE_t>.data,
    qt_static_metacall,
    nullptr,
    qt_staticMetaObjectRelocatingContent<qt_meta_tag_ZN21org_charto_app_pluginE_t>.metaTypes,
    nullptr
} };

void org_charto_app_plugin::qt_static_metacall(QObject *_o, QMetaObject::Call _c, int _id, void **_a)
{
    auto *_t = static_cast<org_charto_app_plugin *>(_o);
    (void)_t;
    (void)_c;
    (void)_id;
    (void)_a;
}

const QMetaObject *org_charto_app_plugin::metaObject() const
{
    return QObject::d_ptr->metaObject ? QObject::d_ptr->dynamicMetaObject() : &staticMetaObject;
}

void *org_charto_app_plugin::qt_metacast(const char *_clname)
{
    if (!_clname) return nullptr;
    if (!strcmp(_clname, qt_staticMetaObjectStaticContent<qt_meta_tag_ZN21org_charto_app_pluginE_t>.strings))
        return static_cast<void*>(this);
    return QQmlEngineExtensionPlugin::qt_metacast(_clname);
}

int org_charto_app_plugin::qt_metacall(QMetaObject::Call _c, int _id, void **_a)
{
    _id = QQmlEngineExtensionPlugin::qt_metacall(_c, _id, _a);
    return _id;
}

#ifdef QT_MOC_EXPORT_PLUGIN_V2
static constexpr unsigned char qt_pluginMetaDataV2_org_charto_app_plugin[] = {
    0xbf, 
    // "IID"
    0x02,  0x78,  0x2e,  'o',  'r',  'g',  '.',  'q', 
    't',  '-',  'p',  'r',  'o',  'j',  'e',  'c', 
    't',  '.',  'Q',  't',  '.',  'Q',  'Q',  'm', 
    'l',  'E',  'n',  'g',  'i',  'n',  'e',  'E', 
    'x',  't',  'e',  'n',  's',  'i',  'o',  'n', 
    'I',  'n',  't',  'e',  'r',  'f',  'a',  'c', 
    'e', 
    // "className"
    0x03,  0x75,  'o',  'r',  'g',  '_',  'c',  'h', 
    'a',  'r',  't',  'o',  '_',  'a',  'p',  'p', 
    '_',  'p',  'l',  'u',  'g',  'i',  'n', 
    // command-line "uri"
    0x63,  'u',  'r',  'i',  0x81,  0x6e,  'o',  'r', 
    'g',  '.',  'c',  'h',  'a',  'r',  't',  'o', 
    '.',  'a',  'p',  'p', 
    0xff, 
};
QT_MOC_EXPORT_PLUGIN_V2(org_charto_app_plugin, org_charto_app_plugin, qt_pluginMetaDataV2_org_charto_app_plugin)
#else
QT_PLUGIN_METADATA_SECTION
Q_CONSTINIT static constexpr unsigned char qt_pluginMetaData_org_charto_app_plugin[] = {
    'Q', 'T', 'M', 'E', 'T', 'A', 'D', 'A', 'T', 'A', ' ', '!',
    // metadata version, Qt version, architectural requirements
    0, QT_VERSION_MAJOR, QT_VERSION_MINOR, qPluginArchRequirements(),
    0xbf, 
    // "IID"
    0x02,  0x78,  0x2e,  'o',  'r',  'g',  '.',  'q', 
    't',  '-',  'p',  'r',  'o',  'j',  'e',  'c', 
    't',  '.',  'Q',  't',  '.',  'Q',  'Q',  'm', 
    'l',  'E',  'n',  'g',  'i',  'n',  'e',  'E', 
    'x',  't',  'e',  'n',  's',  'i',  'o',  'n', 
    'I',  'n',  't',  'e',  'r',  'f',  'a',  'c', 
    'e', 
    // "className"
    0x03,  0x75,  'o',  'r',  'g',  '_',  'c',  'h', 
    'a',  'r',  't',  'o',  '_',  'a',  'p',  'p', 
    '_',  'p',  'l',  'u',  'g',  'i',  'n', 
    // command-line "uri"
    0x63,  'u',  'r',  'i',  0x81,  0x6e,  'o',  'r', 
    'g',  '.',  'c',  'h',  'a',  'r',  't',  'o', 
    '.',  'a',  'p',  'p', 
    0xff, 
};
QT_MOC_EXPORT_PLUGIN(org_charto_app_plugin, org_charto_app_plugin)
#endif  // QT_MOC_EXPORT_PLUGIN_V2

QT_WARNING_POP
