
#include <QtQml/qqmlextensionplugin.h>

// TODO: Add missing handling for GHS (Green Hills Software compiler) that is in
// https://code.qt.io/cgit/qt/qtbase.git/plain/src/corelib/global/qtsymbolmacros.h
extern void qml_register_types_org_charto_app();
extern int qInitResources_qml_module_resources_org_charto_app_qrc();
extern int qInitResources_qmlcache_org_charto_app();

class org_charto_app_plugin : public QQmlEngineExtensionPlugin
{
    Q_OBJECT
    Q_PLUGIN_METADATA(IID "org.qt-project.Qt.QQmlEngineExtensionInterface")

public:
    org_charto_app_plugin(QObject *parent = nullptr) : QQmlEngineExtensionPlugin(parent)
    {
        volatile auto qml_register_types_org_charto_app_usage = &qml_register_types_org_charto_app;
Q_UNUSED(qml_register_types_org_charto_app_usage);
volatile auto qInitResources_qml_module_resources_org_charto_app_qrc_usage = &qInitResources_qml_module_resources_org_charto_app_qrc;
Q_UNUSED(qInitResources_qml_module_resources_org_charto_app_qrc_usage);
volatile auto qInitResources_qmlcache_org_charto_app_usage = &qInitResources_qmlcache_org_charto_app;
Q_UNUSED(qInitResources_qmlcache_org_charto_app_usage);
    }
};

extern "C" {
    // "drive-by initialization" causes the plugin class to be included in static linking scenarios
    // Any function that is called from within this file causes the entire object file to be linked in.
    // Therefore we provide an empty function that can be called at any point to ensure this file is linked in.
    bool init_cxx_qt_qml_module_org_charto_app_plugin() {
        return true;
    }
}

// The moc-generated cpp file doesn't compile on its own; it needs to be #included here.
#include "moc_org_charto_app_plugin.cpp.cpp"
