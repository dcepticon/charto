#include "cxx-qt-lib/qstring.h"
#include <QtCore/QObject>
#include "cxx-qt/connection.h"
#include "charto-app/src/backend.cxxqt.h"
#include <QtQml/QQmlEngine>
#include "cxx-qt/thread.h"
#include <cstddef>
#include <cstdint>
#include <new>
#include <type_traits>
#include <utility>

#ifdef __GNUC__
#pragma GCC diagnostic ignored "-Wmissing-declarations"
#pragma GCC diagnostic ignored "-Wshadow"
#ifdef __clang__
#pragma clang diagnostic ignored "-Wdollar-in-identifier-extension"
#pragma clang diagnostic ignored "-Wreturn-type-c-linkage"
#endif // __clang__
#endif // __GNUC__

namespace rust {
inline namespace cxxbridge1 {
// #include "rust/cxx.h"

#ifndef CXXBRIDGE1_RUST_BOX
#define CXXBRIDGE1_RUST_BOX
template <typename T>
class Box final {
public:
  using element_type = T;
  using const_pointer =
      typename std::add_pointer<typename std::add_const<T>::type>::type;
  using pointer = typename std::add_pointer<T>::type;

  Box() = delete;
  Box(Box &&) noexcept;
  ~Box() noexcept;

  explicit Box(const T &);
  explicit Box(T &&);

  Box &operator=(Box &&) & noexcept;

  const T *operator->() const noexcept;
  const T &operator*() const noexcept;
  T *operator->() noexcept;
  T &operator*() noexcept;

  template <typename... Fields>
  static Box in_place(Fields &&...);

  void swap(Box &) noexcept;

  static Box from_raw(T *) noexcept;

  T *into_raw() noexcept;

  /* Deprecated */ using value_type = element_type;

private:
  class uninit;
  class allocation;
  Box(uninit) noexcept;
  void drop() noexcept;

  friend void swap(Box &lhs, Box &rhs) noexcept { lhs.swap(rhs); }

  T *ptr;
};

template <typename T>
class Box<T>::uninit {};

template <typename T>
class Box<T>::allocation {
  static T *alloc() noexcept;
  static void dealloc(T *) noexcept;

public:
  allocation() noexcept : ptr(alloc()) {}
  ~allocation() noexcept {
    if (this->ptr) {
      dealloc(this->ptr);
    }
  }
  T *ptr;
};

template <typename T>
Box<T>::Box(Box &&other) noexcept : ptr(other.ptr) {
  other.ptr = nullptr;
}

template <typename T>
Box<T>::Box(const T &val) {
  allocation alloc;
  ::new (alloc.ptr) T(val);
  this->ptr = alloc.ptr;
  alloc.ptr = nullptr;
}

template <typename T>
Box<T>::Box(T &&val) {
  allocation alloc;
  ::new (alloc.ptr) T(std::move(val));
  this->ptr = alloc.ptr;
  alloc.ptr = nullptr;
}

template <typename T>
Box<T>::~Box() noexcept {
  if (this->ptr) {
    this->drop();
  }
}

template <typename T>
Box<T> &Box<T>::operator=(Box &&other) & noexcept {
  if (this->ptr) {
    this->drop();
  }
  this->ptr = other.ptr;
  other.ptr = nullptr;
  return *this;
}

template <typename T>
const T *Box<T>::operator->() const noexcept {
  return this->ptr;
}

template <typename T>
const T &Box<T>::operator*() const noexcept {
  return *this->ptr;
}

template <typename T>
T *Box<T>::operator->() noexcept {
  return this->ptr;
}

template <typename T>
T &Box<T>::operator*() noexcept {
  return *this->ptr;
}

template <typename T>
template <typename... Fields>
Box<T> Box<T>::in_place(Fields &&...fields) {
  allocation alloc;
  auto ptr = alloc.ptr;
  ::new (ptr) T{std::forward<Fields>(fields)...};
  alloc.ptr = nullptr;
  return from_raw(ptr);
}

template <typename T>
void Box<T>::swap(Box &rhs) noexcept {
  using std::swap;
  swap(this->ptr, rhs.ptr);
}

template <typename T>
Box<T> Box<T>::from_raw(T *raw) noexcept {
  Box box = uninit{};
  box.ptr = raw;
  return box;
}

template <typename T>
T *Box<T>::into_raw() noexcept {
  T *raw = this->ptr;
  this->ptr = nullptr;
  return raw;
}

template <typename T>
Box<T>::Box(uninit) noexcept {}
#endif // CXXBRIDGE1_RUST_BOX

#ifndef CXXBRIDGE1_RUST_FN
#define CXXBRIDGE1_RUST_FN
template <typename Signature>
class Fn;

template <typename Ret, typename... Args>
class Fn<Ret(Args...)> final {
public:
  Ret operator()(Args... args) const noexcept;
  Fn operator*() const noexcept;

private:
  Ret (*trampoline)(Args..., void *fn) noexcept;
  void *fn;
};

template <typename Ret, typename... Args>
Ret Fn<Ret(Args...)>::operator()(Args... args) const noexcept {
  return (*this->trampoline)(std::forward<Args>(args)..., this->fn);
}

template <typename Ret, typename... Args>
Fn<Ret(Args...)> Fn<Ret(Args...)>::operator*() const noexcept {
  return *this;
}
#endif // CXXBRIDGE1_RUST_FN

#ifndef CXXBRIDGE1_RUST_OPAQUE
#define CXXBRIDGE1_RUST_OPAQUE
class Opaque {
public:
  Opaque() = delete;
  Opaque(const Opaque &) = delete;
  ~Opaque() = delete;
};
#endif // CXXBRIDGE1_RUST_OPAQUE

#ifndef CXXBRIDGE1_IS_COMPLETE
#define CXXBRIDGE1_IS_COMPLETE
namespace detail {
namespace {
template <typename T, typename = std::size_t>
struct is_complete : std::false_type {};
template <typename T>
struct is_complete<T, decltype(sizeof(T))> : std::true_type {};
} // namespace
} // namespace detail
#endif // CXXBRIDGE1_IS_COMPLETE

#ifndef CXXBRIDGE1_LAYOUT
#define CXXBRIDGE1_LAYOUT
class layout {
  template <typename T>
  friend std::size_t size_of();
  template <typename T>
  friend std::size_t align_of();
  template <typename T>
  static typename std::enable_if<std::is_base_of<Opaque, T>::value,
                                 std::size_t>::type
  do_size_of() {
    return T::layout::size();
  }
  template <typename T>
  static typename std::enable_if<!std::is_base_of<Opaque, T>::value,
                                 std::size_t>::type
  do_size_of() {
    return sizeof(T);
  }
  template <typename T>
  static
      typename std::enable_if<detail::is_complete<T>::value, std::size_t>::type
      size_of() {
    return do_size_of<T>();
  }
  template <typename T>
  static typename std::enable_if<std::is_base_of<Opaque, T>::value,
                                 std::size_t>::type
  do_align_of() {
    return T::layout::align();
  }
  template <typename T>
  static typename std::enable_if<!std::is_base_of<Opaque, T>::value,
                                 std::size_t>::type
  do_align_of() {
    return alignof(T);
  }
  template <typename T>
  static
      typename std::enable_if<detail::is_complete<T>::value, std::size_t>::type
      align_of() {
    return do_align_of<T>();
  }
};

template <typename T>
std::size_t size_of() {
  return layout::size_of<T>();
}

template <typename T>
std::size_t align_of() {
  return layout::align_of<T>();
}
#endif // CXXBRIDGE1_LAYOUT

#ifndef CXXBRIDGE1_RELOCATABLE
#define CXXBRIDGE1_RELOCATABLE
namespace detail {
template <typename... Ts>
struct make_void {
  using type = void;
};

template <typename... Ts>
using void_t = typename make_void<Ts...>::type;

template <typename Void, template <typename...> class, typename...>
struct detect : std::false_type {};
template <template <typename...> class T, typename... A>
struct detect<void_t<T<A...>>, T, A...> : std::true_type {};

template <template <typename...> class T, typename... A>
using is_detected = detect<void, T, A...>;

template <typename T>
using detect_IsRelocatable = typename T::IsRelocatable;

template <typename T>
struct get_IsRelocatable
    : std::is_same<typename T::IsRelocatable, std::true_type> {};
} // namespace detail

template <typename T>
struct IsRelocatable
    : std::conditional<
          detail::is_detected<detail::detect_IsRelocatable, T>::value,
          detail::get_IsRelocatable<T>,
          std::integral_constant<
              bool, std::is_trivially_move_constructible<T>::value &&
                        std::is_trivially_destructible<T>::value>>::type {};
#endif // CXXBRIDGE1_RELOCATABLE

namespace detail {
template <typename T, typename = void *>
struct operator_new {
  void *operator()(::std::size_t sz) { return ::operator new(sz); }
};

template <typename T>
struct operator_new<T, decltype(T::operator new(sizeof(T)))> {
  void *operator()(::std::size_t sz) { return T::operator new(sz); }
};
} // namespace detail

template <typename T>
union ManuallyDrop {
  T value;
  ManuallyDrop(T &&value) : value(::std::move(value)) {}
  ~ManuallyDrop() {}
};

template <typename T>
union MaybeUninit {
  T value;
  void *operator new(::std::size_t sz) { return detail::operator_new<T>{}(sz); }
  MaybeUninit() {}
  ~MaybeUninit() {}
};
} // namespace cxxbridge1
} // namespace rust

using ChartBackend = ::ChartBackend;
struct ChartBackendRust;
namespace cxx_qt_ChartBackend {
  struct ChartBackendCxxQtThreadQueuedFn;
}

#ifndef CXXBRIDGE1_STRUCT_ChartBackendRust
#define CXXBRIDGE1_STRUCT_ChartBackendRust
struct ChartBackendRust final : public ::rust::Opaque {
  ~ChartBackendRust() = delete;

private:
  friend ::rust::layout;
  struct layout {
    static ::std::size_t size() noexcept;
    static ::std::size_t align() noexcept;
  };
};
#endif // CXXBRIDGE1_STRUCT_ChartBackendRust

namespace cxx_qt_ChartBackend {
#ifndef CXXBRIDGE1_STRUCT_cxx_qt_ChartBackend$ChartBackendCxxQtThreadQueuedFn
#define CXXBRIDGE1_STRUCT_cxx_qt_ChartBackend$ChartBackendCxxQtThreadQueuedFn
struct ChartBackendCxxQtThreadQueuedFn final : public ::rust::Opaque {
  ~ChartBackendCxxQtThreadQueuedFn() = delete;

private:
  friend ::rust::layout;
  struct layout {
    static ::std::size_t size() noexcept;
    static ::std::size_t align() noexcept;
  };
};
#endif // CXXBRIDGE1_STRUCT_cxx_qt_ChartBackend$ChartBackendCxxQtThreadQueuedFn
} // namespace cxx_qt_ChartBackend

static_assert(
    ::rust::IsRelocatable<::QString>::value,
    "type QString should be trivially move constructible and trivially destructible in C++ to be used as an argument of `set_snapshot`, `set_status`, `set_picked` or return value of `palette_json`, `settings_json`, `print_temp_path` in Rust");
static_assert(
    ::rust::IsRelocatable<::Qt::ConnectionType>::value,
    "type Qt::ConnectionType should be trivially move constructible and trivially destructible in C++ to be used as an argument of `ChartBackend_connect_snapshot_changed`, `ChartBackend_connect_status_changed`, `ChartBackend_connect_picked_changed` in Rust");
static_assert(
    ::rust::IsRelocatable<::rust::cxxqt1::QMetaObjectConnection>::value,
    "type rust::cxxqt1::QMetaObjectConnection should be trivially move constructible and trivially destructible in C++ to be used as a return value of `ChartBackend_connect_snapshot_changed`, `ChartBackend_connect_status_changed`, `ChartBackend_connect_picked_changed` in Rust");
static_assert(
    ::rust::IsRelocatable<::rust::cxxqtgen1::ChartBackendCxxQtSignalHandlersnapshotChanged>::value,
    "type rust::cxxqtgen1::ChartBackendCxxQtSignalHandlersnapshotChanged should be trivially move constructible and trivially destructible in C++ to be used as an argument of `ChartBackend_connect_snapshot_changed`, `drop_ChartBackend_signal_handler_snapshotChanged` in Rust");
static_assert(
    ::rust::IsRelocatable<::rust::cxxqtgen1::ChartBackendCxxQtSignalHandlerstatusChanged>::value,
    "type rust::cxxqtgen1::ChartBackendCxxQtSignalHandlerstatusChanged should be trivially move constructible and trivially destructible in C++ to be used as an argument of `ChartBackend_connect_status_changed`, `drop_ChartBackend_signal_handler_statusChanged` in Rust");
static_assert(
    ::rust::IsRelocatable<::rust::cxxqtgen1::ChartBackendCxxQtSignalHandlerpickedChanged>::value,
    "type rust::cxxqtgen1::ChartBackendCxxQtSignalHandlerpickedChanged should be trivially move constructible and trivially destructible in C++ to be used as an argument of `ChartBackend_connect_picked_changed`, `drop_ChartBackend_signal_handler_pickedChanged` in Rust");
static_assert(
    ::rust::IsRelocatable<::ChartBackendCxxQtThread>::value,
    "type ChartBackendCxxQtThread should be trivially move constructible and trivially destructible in C++ to be used as a return value of `cxx_qt_ffi_ChartBackend_qtThread`, `cxx_qt_ffi_ChartBackend_cxxQtThreadClone` in Rust");

extern "C" {
::std::size_t cxxbridge1$205$ChartBackendRust$operator$sizeof() noexcept;
::std::size_t cxxbridge1$205$ChartBackendRust$operator$alignof() noexcept;

::QString const *cxxbridge1$205$ChartBackend$snapshot(::ChartBackend const &self) noexcept;

void cxxbridge1$205$ChartBackend$set_snapshot(::ChartBackend &self, ::QString *value) noexcept;

::QString const *cxxbridge1$205$ChartBackend$status(::ChartBackend const &self) noexcept;

void cxxbridge1$205$ChartBackend$set_status(::ChartBackend &self, ::QString *value) noexcept;

::QString const *cxxbridge1$205$ChartBackend$picked(::ChartBackend const &self) noexcept;

void cxxbridge1$205$ChartBackend$set_picked(::ChartBackend &self, ::QString *value) noexcept;

void cxxbridge1$205$ChartBackend$snapshot_changed(::ChartBackend &self) noexcept {
  void (::ChartBackend::*snapshot_changed$)() = &::ChartBackend::snapshotChanged;
  (self.*snapshot_changed$)();
}
} // extern "C"

namespace rust {
namespace cxxqtgen1 {
extern "C" {
void rust$cxxqtgen1$cxxbridge1$205$ChartBackend_connect_snapshot_changed(::ChartBackend &self_value, ::rust::cxxqtgen1::ChartBackendCxxQtSignalHandlersnapshotChanged *signal_handler, ::Qt::ConnectionType *conn_type, ::rust::cxxqt1::QMetaObjectConnection *return$) noexcept {
  ::rust::cxxqt1::QMetaObjectConnection (*ChartBackend_connect_snapshot_changed$)(::ChartBackend &, ::rust::cxxqtgen1::ChartBackendCxxQtSignalHandlersnapshotChanged, ::Qt::ConnectionType) = ::rust::cxxqtgen1::ChartBackend_snapshotChangedConnect;
  new (return$) ::rust::cxxqt1::QMetaObjectConnection(ChartBackend_connect_snapshot_changed$(self_value, ::std::move(*signal_handler), ::std::move(*conn_type)));
}

void rust$cxxqtgen1$cxxbridge1$205$drop_ChartBackend_signal_handler_snapshotChanged(::rust::cxxqtgen1::ChartBackendCxxQtSignalHandlersnapshotChanged *handler) noexcept;

void rust$cxxqtgen1$cxxbridge1$205$call_ChartBackend_signal_handler_snapshotChanged(::rust::cxxqtgen1::ChartBackendCxxQtSignalHandlersnapshotChanged &handler, ::ChartBackend &self_value) noexcept;
} // extern "C"
} // namespace cxxqtgen1
} // namespace rust

extern "C" {
void cxxbridge1$205$ChartBackend$status_changed(::ChartBackend &self) noexcept {
  void (::ChartBackend::*status_changed$)() = &::ChartBackend::statusChanged;
  (self.*status_changed$)();
}
} // extern "C"

namespace rust {
namespace cxxqtgen1 {
extern "C" {
void rust$cxxqtgen1$cxxbridge1$205$ChartBackend_connect_status_changed(::ChartBackend &self_value, ::rust::cxxqtgen1::ChartBackendCxxQtSignalHandlerstatusChanged *signal_handler, ::Qt::ConnectionType *conn_type, ::rust::cxxqt1::QMetaObjectConnection *return$) noexcept {
  ::rust::cxxqt1::QMetaObjectConnection (*ChartBackend_connect_status_changed$)(::ChartBackend &, ::rust::cxxqtgen1::ChartBackendCxxQtSignalHandlerstatusChanged, ::Qt::ConnectionType) = ::rust::cxxqtgen1::ChartBackend_statusChangedConnect;
  new (return$) ::rust::cxxqt1::QMetaObjectConnection(ChartBackend_connect_status_changed$(self_value, ::std::move(*signal_handler), ::std::move(*conn_type)));
}

void rust$cxxqtgen1$cxxbridge1$205$drop_ChartBackend_signal_handler_statusChanged(::rust::cxxqtgen1::ChartBackendCxxQtSignalHandlerstatusChanged *handler) noexcept;

void rust$cxxqtgen1$cxxbridge1$205$call_ChartBackend_signal_handler_statusChanged(::rust::cxxqtgen1::ChartBackendCxxQtSignalHandlerstatusChanged &handler, ::ChartBackend &self_value) noexcept;
} // extern "C"
} // namespace cxxqtgen1
} // namespace rust

extern "C" {
void cxxbridge1$205$ChartBackend$picked_changed(::ChartBackend &self) noexcept {
  void (::ChartBackend::*picked_changed$)() = &::ChartBackend::pickedChanged;
  (self.*picked_changed$)();
}
} // extern "C"

namespace rust {
namespace cxxqtgen1 {
extern "C" {
void rust$cxxqtgen1$cxxbridge1$205$ChartBackend_connect_picked_changed(::ChartBackend &self_value, ::rust::cxxqtgen1::ChartBackendCxxQtSignalHandlerpickedChanged *signal_handler, ::Qt::ConnectionType *conn_type, ::rust::cxxqt1::QMetaObjectConnection *return$) noexcept {
  ::rust::cxxqt1::QMetaObjectConnection (*ChartBackend_connect_picked_changed$)(::ChartBackend &, ::rust::cxxqtgen1::ChartBackendCxxQtSignalHandlerpickedChanged, ::Qt::ConnectionType) = ::rust::cxxqtgen1::ChartBackend_pickedChangedConnect;
  new (return$) ::rust::cxxqt1::QMetaObjectConnection(ChartBackend_connect_picked_changed$(self_value, ::std::move(*signal_handler), ::std::move(*conn_type)));
}

void rust$cxxqtgen1$cxxbridge1$205$drop_ChartBackend_signal_handler_pickedChanged(::rust::cxxqtgen1::ChartBackendCxxQtSignalHandlerpickedChanged *handler) noexcept;

void rust$cxxqtgen1$cxxbridge1$205$call_ChartBackend_signal_handler_pickedChanged(::rust::cxxqtgen1::ChartBackendCxxQtSignalHandlerpickedChanged &handler, ::ChartBackend &self_value) noexcept;
} // extern "C"
} // namespace cxxqtgen1
} // namespace rust

extern "C" {
void cxxbridge1$205$ChartBackend$palette_json(::ChartBackend const &self, ::QString *return$) noexcept;

void cxxbridge1$205$ChartBackend$new_chart(::ChartBackend &self, ::std::int32_t rows, ::std::int32_t cols) noexcept;

void cxxbridge1$205$ChartBackend$open_file(::ChartBackend &self, ::QString const &path) noexcept;

void cxxbridge1$205$ChartBackend$save_file(::ChartBackend &self, ::QString const &path) noexcept;

void cxxbridge1$205$ChartBackend$set_active_symbol(::ChartBackend &self, ::QString const &name) noexcept;

void cxxbridge1$205$ChartBackend$set_color(::ChartBackend &self, ::QString const &hex) noexcept;

void cxxbridge1$205$ChartBackend$add_color(::ChartBackend &self, ::QString const &hex) noexcept;

void cxxbridge1$205$ChartBackend$set_mode(::ChartBackend &self, ::std::int32_t mode) noexcept;

void cxxbridge1$205$ChartBackend$click_cell(::ChartBackend &self, ::std::int32_t row, ::std::int32_t col) noexcept;

void cxxbridge1$205$ChartBackend$select_rect(::ChartBackend &self, ::std::int32_t col, ::std::int32_t row, ::std::int32_t num_cols, ::std::int32_t num_rows) noexcept;

void cxxbridge1$205$ChartBackend$select_row(::ChartBackend &self, ::std::int32_t row) noexcept;

void cxxbridge1$205$ChartBackend$select_column(::ChartBackend &self, ::std::int32_t col) noexcept;

void cxxbridge1$205$ChartBackend$clear_selection(::ChartBackend &self) noexcept;

void cxxbridge1$205$ChartBackend$apply_color(::ChartBackend &self) noexcept;

void cxxbridge1$205$ChartBackend$undo(::ChartBackend &self) noexcept;

void cxxbridge1$205$ChartBackend$redo(::ChartBackend &self) noexcept;

void cxxbridge1$205$ChartBackend$copy(::ChartBackend &self) noexcept;

void cxxbridge1$205$ChartBackend$paste(::ChartBackend &self, ::std::int32_t col, ::std::int32_t row) noexcept;

void cxxbridge1$205$ChartBackend$insert_rows(::ChartBackend &self, ::std::int32_t num, bool above) noexcept;

void cxxbridge1$205$ChartBackend$delete_rows(::ChartBackend &self) noexcept;

void cxxbridge1$205$ChartBackend$insert_columns(::ChartBackend &self, ::std::int32_t num, bool left) noexcept;

void cxxbridge1$205$ChartBackend$delete_columns(::ChartBackend &self) noexcept;

void cxxbridge1$205$ChartBackend$add_row_repeat(::ChartBackend &self, ::std::int32_t multiplicity) noexcept;

void cxxbridge1$205$ChartBackend$delete_row_repeat(::ChartBackend &self) noexcept;

bool cxxbridge1$205$ChartBackend$has_recovery(::ChartBackend const &self, ::QString const &path) noexcept;

void cxxbridge1$205$ChartBackend$open_recovery(::ChartBackend &self, ::QString const &path) noexcept;

void cxxbridge1$205$ChartBackend$autosave(::ChartBackend &self) noexcept;

void cxxbridge1$205$ChartBackend$end_session(::ChartBackend &self) noexcept;

void cxxbridge1$205$ChartBackend$clear_recent_files(::ChartBackend &self) noexcept;

void cxxbridge1$205$ChartBackend$select_same_color(::ChartBackend &self, ::std::int32_t row, ::std::int32_t col) noexcept;

void cxxbridge1$205$ChartBackend$select_same_symbol(::ChartBackend &self, ::std::int32_t row, ::std::int32_t col) noexcept;

void cxxbridge1$205$ChartBackend$set_row_label(::ChartBackend &self, ::std::int32_t row, ::QString const &text) noexcept;

void cxxbridge1$205$ChartBackend$set_column_label(::ChartBackend &self, ::std::int32_t col, ::QString const &text) noexcept;

void cxxbridge1$205$ChartBackend$settings_json(::ChartBackend const &self, ::QString *return$) noexcept;

void cxxbridge1$205$ChartBackend$apply_settings(::ChartBackend &self, ::QString const &json) noexcept;

void cxxbridge1$205$ChartBackend$export_svg(::ChartBackend &self, ::QString const &path) noexcept;

void cxxbridge1$205$ChartBackend$print_temp_path(::ChartBackend const &self, ::QString *return$) noexcept;

void cxxbridge1$205$ChartBackend$translate(::ChartBackend const &self, ::QString const &text, ::QString *return$) noexcept;

void cxxbridge1$205$ChartBackend$startup_file(::ChartBackend const &self, ::QString *return$) noexcept;

void cxxbridge1$205$ChartBackend$print_image(::ChartBackend &self, ::QString const &png_path, ::QString const &title) noexcept;

void cxxbridge1$205$ChartBackend$choose_file(::ChartBackend &self, ::QString const &purpose, ::QString const &current) noexcept;

void cxxbridge1$205$ChartBackend$report_status(::ChartBackend &self, ::QString const &message) noexcept;
} // extern "C"

namespace rust {
namespace cxxqt1 {
extern "C" {
void rust$cxxqt1$cxxbridge1$205$cxx_qt_ffi_ChartBackend_qtThread(::ChartBackend const &qobject, ::ChartBackendCxxQtThread *return$) noexcept {
  ::ChartBackendCxxQtThread (*cxx_qt_ffi_ChartBackend_qtThread$)(::ChartBackend const &) = ::rust::cxxqt1::qtThread;
  new (return$) ::ChartBackendCxxQtThread(cxx_qt_ffi_ChartBackend_qtThread$(qobject));
}

::std::uint8_t rust$cxxqt1$cxxbridge1$205$cxx_qt_ffi_ChartBackend_cxxQtThreadQueue(::ChartBackendCxxQtThread const &cxx_qt_thread, ::rust::Fn<void(::ChartBackend &, ::rust::Box<::cxx_qt_ChartBackend::ChartBackendCxxQtThreadQueuedFn>)> func, ::cxx_qt_ChartBackend::ChartBackendCxxQtThreadQueuedFn *arg) noexcept {
  ::std::uint8_t (*cxx_qt_ffi_ChartBackend_cxxQtThreadQueue$)(::ChartBackendCxxQtThread const &, ::rust::Fn<void(::ChartBackend &, ::rust::Box<::cxx_qt_ChartBackend::ChartBackendCxxQtThreadQueuedFn>)>, ::rust::Box<::cxx_qt_ChartBackend::ChartBackendCxxQtThreadQueuedFn>) = ::rust::cxxqt1::cxxQtThreadQueue;
  return cxx_qt_ffi_ChartBackend_cxxQtThreadQueue$(cxx_qt_thread, func, ::rust::Box<::cxx_qt_ChartBackend::ChartBackendCxxQtThreadQueuedFn>::from_raw(arg));
}

void rust$cxxqt1$cxxbridge1$205$cxx_qt_ffi_ChartBackend_cxxQtThreadQueue$func$1(::ChartBackend &arg0, ::cxx_qt_ChartBackend::ChartBackendCxxQtThreadQueuedFn *arg1, void *) noexcept;

void rust$cxxqt1$cxxbridge1$205$cxx_qt_ffi_ChartBackend_cxxQtThreadQueue$func$0(::ChartBackend &arg0, ::rust::Box<::cxx_qt_ChartBackend::ChartBackendCxxQtThreadQueuedFn> arg1, void *extern$) noexcept {
  rust$cxxqt1$cxxbridge1$205$cxx_qt_ffi_ChartBackend_cxxQtThreadQueue$func$1(arg0, arg1.into_raw(), extern$);
}

void rust$cxxqt1$cxxbridge1$205$cxx_qt_ffi_ChartBackend_cxxQtThreadClone(::ChartBackendCxxQtThread const &cxx_qt_thread, ::ChartBackendCxxQtThread *return$) noexcept {
  ::ChartBackendCxxQtThread (*cxx_qt_ffi_ChartBackend_cxxQtThreadClone$)(::ChartBackendCxxQtThread const &) = ::rust::cxxqt1::cxxQtThreadClone;
  new (return$) ::ChartBackendCxxQtThread(cxx_qt_ffi_ChartBackend_cxxQtThreadClone$(cxx_qt_thread));
}

void rust$cxxqt1$cxxbridge1$205$cxx_qt_ffi_ChartBackend_cxxQtThreadDrop(::ChartBackendCxxQtThread &cxx_qt_thread) noexcept {
  void (*cxx_qt_ffi_ChartBackend_cxxQtThreadDrop$)(::ChartBackendCxxQtThread &) = ::rust::cxxqt1::cxxQtThreadDrop;
  cxx_qt_ffi_ChartBackend_cxxQtThreadDrop$(cxx_qt_thread);
}

bool rust$cxxqt1$cxxbridge1$205$cxx_qt_ffi_ChartBackend_cxxQtThreadIsDestroyed(::ChartBackendCxxQtThread const &cxx_qt_thread) noexcept {
  bool (*cxx_qt_ffi_ChartBackend_cxxQtThreadIsDestroyed$)(::ChartBackendCxxQtThread const &) = ::rust::cxxqt1::cxxQtThreadIsDestroyed;
  return cxx_qt_ffi_ChartBackend_cxxQtThreadIsDestroyed$(cxx_qt_thread);
}
} // extern "C"
} // namespace cxxqt1
} // namespace rust

namespace cxx_qt_ChartBackend {
extern "C" {
::std::size_t cxx_qt_ChartBackend$cxxbridge1$205$ChartBackendCxxQtThreadQueuedFn$operator$sizeof() noexcept;
::std::size_t cxx_qt_ChartBackend$cxxbridge1$205$ChartBackendCxxQtThreadQueuedFn$operator$alignof() noexcept;
} // extern "C"
} // namespace cxx_qt_ChartBackend

namespace rust {
namespace cxxqt1 {
extern "C" {
::QObject const *rust$cxxqt1$cxxbridge1$205$cxx_qt_ffi_ChartBackend_upcastPtr(::ChartBackend const *thiz) noexcept {
  ::QObject const *(*cxx_qt_ffi_ChartBackend_upcastPtr$)(::ChartBackend const *) = ::rust::cxxqt1::upcastPtr;
  return cxx_qt_ffi_ChartBackend_upcastPtr$(thiz);
}

::ChartBackend const *rust$cxxqt1$cxxbridge1$205$cxx_qt_ffi_ChartBackend_downcastPtr(::QObject const *base) noexcept {
  ::ChartBackend const *(*cxx_qt_ffi_ChartBackend_downcastPtr$)(::QObject const *) = ::rust::cxxqt1::downcastPtr;
  return cxx_qt_ffi_ChartBackend_downcastPtr$(base);
}
} // extern "C"
} // namespace cxxqt1
} // namespace rust

namespace cxx_qt_ChartBackend {
extern "C" {
::ChartBackendRust *cxx_qt_ChartBackend$cxxbridge1$205$create_rs_ChartBackendRust() noexcept;
} // extern "C"
} // namespace cxx_qt_ChartBackend

namespace rust {
namespace cxxqt1 {
extern "C" {
void rust$cxxqt1$cxxbridge1$205$cxx_qt_ffi_ChartBackend_unsafeRust(::ChartBackend const &outer, ::ChartBackendRust const **return$) noexcept {
  ::ChartBackendRust const &(*cxx_qt_ffi_ChartBackend_unsafeRust$)(::ChartBackend const &) = ::rust::cxxqt1::unsafeRust;
  new (return$) ::ChartBackendRust const *(&cxx_qt_ffi_ChartBackend_unsafeRust$(outer));
}

void rust$cxxqt1$cxxbridge1$205$cxx_qt_ffi_ChartBackend_unsafeRustMut(::ChartBackend &outer, ::ChartBackendRust **return$) noexcept {
  ::ChartBackendRust &(*cxx_qt_ffi_ChartBackend_unsafeRustMut$)(::ChartBackend &) = ::rust::cxxqt1::unsafeRustMut;
  new (return$) ::ChartBackendRust *(&cxx_qt_ffi_ChartBackend_unsafeRustMut$(outer));
}
} // extern "C"
} // namespace cxxqt1
} // namespace rust

::std::size_t ChartBackendRust::layout::size() noexcept {
  return cxxbridge1$205$ChartBackendRust$operator$sizeof();
}

::std::size_t ChartBackendRust::layout::align() noexcept {
  return cxxbridge1$205$ChartBackendRust$operator$alignof();
}

::QString const &ChartBackend::getSnapshot() const noexcept {
  return *cxxbridge1$205$ChartBackend$snapshot(*this);
}

void ChartBackend::setSnapshot(::QString value) noexcept {
  ::rust::ManuallyDrop<::QString> value$(::std::move(value));
  cxxbridge1$205$ChartBackend$set_snapshot(*this, &value$.value);
}

::QString const &ChartBackend::getStatus() const noexcept {
  return *cxxbridge1$205$ChartBackend$status(*this);
}

void ChartBackend::setStatus(::QString value) noexcept {
  ::rust::ManuallyDrop<::QString> value$(::std::move(value));
  cxxbridge1$205$ChartBackend$set_status(*this, &value$.value);
}

::QString const &ChartBackend::getPicked() const noexcept {
  return *cxxbridge1$205$ChartBackend$picked(*this);
}

void ChartBackend::setPicked(::QString value) noexcept {
  ::rust::ManuallyDrop<::QString> value$(::std::move(value));
  cxxbridge1$205$ChartBackend$set_picked(*this, &value$.value);
}

namespace rust {
namespace cxxqtgen1 {
void drop_ChartBackend_signal_handler_snapshotChanged(::rust::cxxqtgen1::ChartBackendCxxQtSignalHandlersnapshotChanged handler) noexcept {
  ::rust::ManuallyDrop<::rust::cxxqtgen1::ChartBackendCxxQtSignalHandlersnapshotChanged> handler$(::std::move(handler));
  rust$cxxqtgen1$cxxbridge1$205$drop_ChartBackend_signal_handler_snapshotChanged(&handler$.value);
}

void call_ChartBackend_signal_handler_snapshotChanged(::rust::cxxqtgen1::ChartBackendCxxQtSignalHandlersnapshotChanged &handler, ::ChartBackend &self_value) noexcept {
  rust$cxxqtgen1$cxxbridge1$205$call_ChartBackend_signal_handler_snapshotChanged(handler, self_value);
}

void drop_ChartBackend_signal_handler_statusChanged(::rust::cxxqtgen1::ChartBackendCxxQtSignalHandlerstatusChanged handler) noexcept {
  ::rust::ManuallyDrop<::rust::cxxqtgen1::ChartBackendCxxQtSignalHandlerstatusChanged> handler$(::std::move(handler));
  rust$cxxqtgen1$cxxbridge1$205$drop_ChartBackend_signal_handler_statusChanged(&handler$.value);
}

void call_ChartBackend_signal_handler_statusChanged(::rust::cxxqtgen1::ChartBackendCxxQtSignalHandlerstatusChanged &handler, ::ChartBackend &self_value) noexcept {
  rust$cxxqtgen1$cxxbridge1$205$call_ChartBackend_signal_handler_statusChanged(handler, self_value);
}

void drop_ChartBackend_signal_handler_pickedChanged(::rust::cxxqtgen1::ChartBackendCxxQtSignalHandlerpickedChanged handler) noexcept {
  ::rust::ManuallyDrop<::rust::cxxqtgen1::ChartBackendCxxQtSignalHandlerpickedChanged> handler$(::std::move(handler));
  rust$cxxqtgen1$cxxbridge1$205$drop_ChartBackend_signal_handler_pickedChanged(&handler$.value);
}

void call_ChartBackend_signal_handler_pickedChanged(::rust::cxxqtgen1::ChartBackendCxxQtSignalHandlerpickedChanged &handler, ::ChartBackend &self_value) noexcept {
  rust$cxxqtgen1$cxxbridge1$205$call_ChartBackend_signal_handler_pickedChanged(handler, self_value);
}
} // namespace cxxqtgen1
} // namespace rust

::QString ChartBackend::paletteJson() const noexcept {
  ::rust::MaybeUninit<::QString> return$;
  cxxbridge1$205$ChartBackend$palette_json(*this, &return$.value);
  return ::std::move(return$.value);
}

void ChartBackend::newChart(::std::int32_t rows, ::std::int32_t cols) noexcept {
  cxxbridge1$205$ChartBackend$new_chart(*this, rows, cols);
}

void ChartBackend::openFile(::QString const &path) noexcept {
  cxxbridge1$205$ChartBackend$open_file(*this, path);
}

void ChartBackend::saveFile(::QString const &path) noexcept {
  cxxbridge1$205$ChartBackend$save_file(*this, path);
}

void ChartBackend::setActiveSymbol(::QString const &name) noexcept {
  cxxbridge1$205$ChartBackend$set_active_symbol(*this, name);
}

void ChartBackend::setColor(::QString const &hex) noexcept {
  cxxbridge1$205$ChartBackend$set_color(*this, hex);
}

void ChartBackend::addColor(::QString const &hex) noexcept {
  cxxbridge1$205$ChartBackend$add_color(*this, hex);
}

void ChartBackend::setMode(::std::int32_t mode) noexcept {
  cxxbridge1$205$ChartBackend$set_mode(*this, mode);
}

void ChartBackend::clickCell(::std::int32_t row, ::std::int32_t col) noexcept {
  cxxbridge1$205$ChartBackend$click_cell(*this, row, col);
}

void ChartBackend::selectRect(::std::int32_t col, ::std::int32_t row, ::std::int32_t num_cols, ::std::int32_t num_rows) noexcept {
  cxxbridge1$205$ChartBackend$select_rect(*this, col, row, num_cols, num_rows);
}

void ChartBackend::selectRow(::std::int32_t row) noexcept {
  cxxbridge1$205$ChartBackend$select_row(*this, row);
}

void ChartBackend::selectColumn(::std::int32_t col) noexcept {
  cxxbridge1$205$ChartBackend$select_column(*this, col);
}

void ChartBackend::clearSelection() noexcept {
  cxxbridge1$205$ChartBackend$clear_selection(*this);
}

void ChartBackend::applyColor() noexcept {
  cxxbridge1$205$ChartBackend$apply_color(*this);
}

void ChartBackend::undo() noexcept {
  cxxbridge1$205$ChartBackend$undo(*this);
}

void ChartBackend::redo() noexcept {
  cxxbridge1$205$ChartBackend$redo(*this);
}

void ChartBackend::copy() noexcept {
  cxxbridge1$205$ChartBackend$copy(*this);
}

void ChartBackend::paste(::std::int32_t col, ::std::int32_t row) noexcept {
  cxxbridge1$205$ChartBackend$paste(*this, col, row);
}

void ChartBackend::insertRows(::std::int32_t num, bool above) noexcept {
  cxxbridge1$205$ChartBackend$insert_rows(*this, num, above);
}

void ChartBackend::deleteRows() noexcept {
  cxxbridge1$205$ChartBackend$delete_rows(*this);
}

void ChartBackend::insertColumns(::std::int32_t num, bool left) noexcept {
  cxxbridge1$205$ChartBackend$insert_columns(*this, num, left);
}

void ChartBackend::deleteColumns() noexcept {
  cxxbridge1$205$ChartBackend$delete_columns(*this);
}

void ChartBackend::addRowRepeat(::std::int32_t multiplicity) noexcept {
  cxxbridge1$205$ChartBackend$add_row_repeat(*this, multiplicity);
}

void ChartBackend::deleteRowRepeat() noexcept {
  cxxbridge1$205$ChartBackend$delete_row_repeat(*this);
}

bool ChartBackend::hasRecovery(::QString const &path) const noexcept {
  return cxxbridge1$205$ChartBackend$has_recovery(*this, path);
}

void ChartBackend::openRecovery(::QString const &path) noexcept {
  cxxbridge1$205$ChartBackend$open_recovery(*this, path);
}

void ChartBackend::autosave() noexcept {
  cxxbridge1$205$ChartBackend$autosave(*this);
}

void ChartBackend::endSession() noexcept {
  cxxbridge1$205$ChartBackend$end_session(*this);
}

void ChartBackend::clearRecentFiles() noexcept {
  cxxbridge1$205$ChartBackend$clear_recent_files(*this);
}

void ChartBackend::selectSameColor(::std::int32_t row, ::std::int32_t col) noexcept {
  cxxbridge1$205$ChartBackend$select_same_color(*this, row, col);
}

void ChartBackend::selectSameSymbol(::std::int32_t row, ::std::int32_t col) noexcept {
  cxxbridge1$205$ChartBackend$select_same_symbol(*this, row, col);
}

void ChartBackend::setRowLabel(::std::int32_t row, ::QString const &text) noexcept {
  cxxbridge1$205$ChartBackend$set_row_label(*this, row, text);
}

void ChartBackend::setColumnLabel(::std::int32_t col, ::QString const &text) noexcept {
  cxxbridge1$205$ChartBackend$set_column_label(*this, col, text);
}

::QString ChartBackend::settingsJson() const noexcept {
  ::rust::MaybeUninit<::QString> return$;
  cxxbridge1$205$ChartBackend$settings_json(*this, &return$.value);
  return ::std::move(return$.value);
}

void ChartBackend::applySettings(::QString const &json) noexcept {
  cxxbridge1$205$ChartBackend$apply_settings(*this, json);
}

void ChartBackend::exportSvg(::QString const &path) noexcept {
  cxxbridge1$205$ChartBackend$export_svg(*this, path);
}

::QString ChartBackend::printTempPath() const noexcept {
  ::rust::MaybeUninit<::QString> return$;
  cxxbridge1$205$ChartBackend$print_temp_path(*this, &return$.value);
  return ::std::move(return$.value);
}

::QString ChartBackend::translate(::QString const &text) const noexcept {
  ::rust::MaybeUninit<::QString> return$;
  cxxbridge1$205$ChartBackend$translate(*this, text, &return$.value);
  return ::std::move(return$.value);
}

::QString ChartBackend::startupFile() const noexcept {
  ::rust::MaybeUninit<::QString> return$;
  cxxbridge1$205$ChartBackend$startup_file(*this, &return$.value);
  return ::std::move(return$.value);
}

void ChartBackend::printImage(::QString const &png_path, ::QString const &title) noexcept {
  cxxbridge1$205$ChartBackend$print_image(*this, png_path, title);
}

void ChartBackend::chooseFile(::QString const &purpose, ::QString const &current) noexcept {
  cxxbridge1$205$ChartBackend$choose_file(*this, purpose, current);
}

void ChartBackend::reportStatus(::QString const &message) noexcept {
  cxxbridge1$205$ChartBackend$report_status(*this, message);
}

namespace cxx_qt_ChartBackend {
::std::size_t ChartBackendCxxQtThreadQueuedFn::layout::size() noexcept {
  return cxx_qt_ChartBackend$cxxbridge1$205$ChartBackendCxxQtThreadQueuedFn$operator$sizeof();
}

::std::size_t ChartBackendCxxQtThreadQueuedFn::layout::align() noexcept {
  return cxx_qt_ChartBackend$cxxbridge1$205$ChartBackendCxxQtThreadQueuedFn$operator$alignof();
}

::rust::Box<::ChartBackendRust> createRs() noexcept {
  return ::rust::Box<::ChartBackendRust>::from_raw(cxx_qt_ChartBackend$cxxbridge1$205$create_rs_ChartBackendRust());
}
} // namespace cxx_qt_ChartBackend

extern "C" {
::cxx_qt_ChartBackend::ChartBackendCxxQtThreadQueuedFn *cxxbridge1$box$cxx_qt_ChartBackend$ChartBackendCxxQtThreadQueuedFn$alloc() noexcept;
void cxxbridge1$box$cxx_qt_ChartBackend$ChartBackendCxxQtThreadQueuedFn$dealloc(::cxx_qt_ChartBackend::ChartBackendCxxQtThreadQueuedFn *) noexcept;
void cxxbridge1$box$cxx_qt_ChartBackend$ChartBackendCxxQtThreadQueuedFn$drop(::rust::Box<::cxx_qt_ChartBackend::ChartBackendCxxQtThreadQueuedFn> *ptr) noexcept;

::ChartBackendRust *cxxbridge1$box$ChartBackendRust$alloc() noexcept;
void cxxbridge1$box$ChartBackendRust$dealloc(::ChartBackendRust *) noexcept;
void cxxbridge1$box$ChartBackendRust$drop(::rust::Box<::ChartBackendRust> *ptr) noexcept;
} // extern "C"

namespace rust {
inline namespace cxxbridge1 {
template <>
::cxx_qt_ChartBackend::ChartBackendCxxQtThreadQueuedFn *Box<::cxx_qt_ChartBackend::ChartBackendCxxQtThreadQueuedFn>::allocation::alloc() noexcept {
  return cxxbridge1$box$cxx_qt_ChartBackend$ChartBackendCxxQtThreadQueuedFn$alloc();
}
template <>
void Box<::cxx_qt_ChartBackend::ChartBackendCxxQtThreadQueuedFn>::allocation::dealloc(::cxx_qt_ChartBackend::ChartBackendCxxQtThreadQueuedFn *ptr) noexcept {
  cxxbridge1$box$cxx_qt_ChartBackend$ChartBackendCxxQtThreadQueuedFn$dealloc(ptr);
}
template <>
void Box<::cxx_qt_ChartBackend::ChartBackendCxxQtThreadQueuedFn>::drop() noexcept {
  cxxbridge1$box$cxx_qt_ChartBackend$ChartBackendCxxQtThreadQueuedFn$drop(this);
}
template <>
::ChartBackendRust *Box<::ChartBackendRust>::allocation::alloc() noexcept {
  return cxxbridge1$box$ChartBackendRust$alloc();
}
template <>
void Box<::ChartBackendRust>::allocation::dealloc(::ChartBackendRust *ptr) noexcept {
  cxxbridge1$box$ChartBackendRust$dealloc(ptr);
}
template <>
void Box<::ChartBackendRust>::drop() noexcept {
  cxxbridge1$box$ChartBackendRust$drop(this);
}
} // namespace cxxbridge1
} // namespace rust
