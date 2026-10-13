
#include <mutex>

extern "C" bool cxx_qt_init_crate_cxx_qt();
extern "C" bool cxx_qt_init_crate_cxx_qt_lib();
extern "C" bool cxx_qt_init_qml_module_org_charto_app();

static bool do_init() {
    static std::once_flag flag;
    std::call_once(flag, []() {
        cxx_qt_init_crate_cxx_qt();
cxx_qt_init_crate_cxx_qt_lib();
cxx_qt_init_qml_module_org_charto_app();
    });
    return true;
}

extern "C" bool cxx_qt_init_crate_charto_app() {
    return do_init();
}
            