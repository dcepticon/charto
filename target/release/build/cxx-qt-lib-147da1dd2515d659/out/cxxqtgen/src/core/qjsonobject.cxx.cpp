#include "cxx-qt-lib/qjsonobject.h"
#include "cxx-qt-lib/qjsonvalue.h"
#include "cxx-qt-lib/qstring.h"
#include "cxx-qt-lib/qstringlist.h"
#include "cxx-qt-lib/common.h"
#include <new>
#include <type_traits>
#if defined(_WIN32)
#include <basetsd.h>
#else
#include <sys/types.h>
#endif

#ifdef __GNUC__
#pragma GCC diagnostic ignored "-Wmissing-declarations"
#ifdef __clang__
#pragma clang diagnostic ignored "-Wdollar-in-identifier-extension"
#endif // __clang__
#endif // __GNUC__

namespace rust {
inline namespace cxxbridge1 {
// #include "rust/cxx.h"

#ifndef CXXBRIDGE1_RUST_ISIZE
#define CXXBRIDGE1_RUST_ISIZE
#if defined(_WIN32)
using isize = SSIZE_T;
#else
using isize = ssize_t;
#endif
#endif // CXXBRIDGE1_RUST_ISIZE

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
} // namespace cxxbridge1
} // namespace rust

static_assert(
    ::rust::IsRelocatable<::QJsonObject>::value,
    "type QJsonObject should be trivially move constructible and trivially destructible in C++ to be used as a return value of `qjsonobject_init_default`, `qjsonobject_init_from_qjsonobject` in Rust");
static_assert(
    ::rust::IsRelocatable<::QJsonValue>::value,
    "type QJsonValue should be trivially move constructible and trivially destructible in C++ to be used as a return value of `value` in Rust");
static_assert(
    ::rust::IsRelocatable<::QString>::value,
    "type QString should be trivially move constructible and trivially destructible in C++ to be used as a return value of `qjsonobject_to_debug_qstring` in Rust");
static_assert(
    ::rust::IsRelocatable<::QStringList>::value,
    "type QStringList should be trivially move constructible and trivially destructible in C++ to be used as a return value of `keys` in Rust");

extern "C" {
bool cxxbridge1$205$QJsonObject$is_empty(::QJsonObject const &self) noexcept {
  bool (::QJsonObject::*is_empty$)() const = &::QJsonObject::isEmpty;
  return (self.*is_empty$)();
}

bool cxxbridge1$205$QJsonObject$contains(::QJsonObject const &self, ::QString const &key) noexcept {
  bool (::QJsonObject::*contains$)(::QString const &) const = &::QJsonObject::contains;
  return (self.*contains$)(key);
}

void cxxbridge1$205$QJsonObject$value(::QJsonObject const &self, ::QString const &key, ::QJsonValue *return$) noexcept {
  ::QJsonValue (::QJsonObject::*value$)(::QString const &) const = &::QJsonObject::value;
  new (return$) ::QJsonValue((self.*value$)(key));
}

void cxxbridge1$205$QJsonObject$remove(::QJsonObject &self, ::QString const &key) noexcept {
  void (::QJsonObject::*remove$)(::QString const &) = &::QJsonObject::remove;
  (self.*remove$)(key);
}

void cxxbridge1$205$QJsonObject$keys(::QJsonObject const &self, ::QStringList *return$) noexcept {
  ::QStringList (::QJsonObject::*keys$)() const = &::QJsonObject::keys;
  new (return$) ::QStringList((self.*keys$)());
}
} // extern "C"

namespace rust {
namespace cxxqtlib1 {
extern "C" {
void rust$cxxqtlib1$cxxbridge1$205$qjsonobject_drop(::QJsonObject &object) noexcept {
  void (*qjsonobject_drop$)(::QJsonObject &) = ::rust::cxxqtlib1::drop;
  qjsonobject_drop$(object);
}

void rust$cxxqtlib1$cxxbridge1$205$qjsonobject_init_default(::QJsonObject *return$) noexcept {
  ::QJsonObject (*qjsonobject_init_default$)() = ::rust::cxxqtlib1::construct;
  new (return$) ::QJsonObject(qjsonobject_init_default$());
}

void rust$cxxqtlib1$cxxbridge1$205$qjsonobject_init_from_qjsonobject(::QJsonObject const &object, ::QJsonObject *return$) noexcept {
  ::QJsonObject (*qjsonobject_init_from_qjsonobject$)(::QJsonObject const &) = ::rust::cxxqtlib1::construct;
  new (return$) ::QJsonObject(qjsonobject_init_from_qjsonobject$(object));
}

bool rust$cxxqtlib1$cxxbridge1$205$qjsonobject_eq(::QJsonObject const &a, ::QJsonObject const &b) noexcept {
  bool (*qjsonobject_eq$)(::QJsonObject const &, ::QJsonObject const &) = ::rust::cxxqtlib1::operatorEq;
  return qjsonobject_eq$(a, b);
}

void rust$cxxqtlib1$cxxbridge1$205$qjsonobject_to_debug_qstring(::QJsonObject const &object, ::QString *return$) noexcept {
  ::QString (*qjsonobject_to_debug_qstring$)(::QJsonObject const &) = ::rust::cxxqtlib1::toDebugQString;
  new (return$) ::QString(qjsonobject_to_debug_qstring$(object));
}

::rust::isize rust$cxxqtlib1$cxxbridge1$205$qjsonobject_len(::QJsonObject const &object) noexcept {
  ::rust::isize (*qjsonobject_len$)(::QJsonObject const &) = ::rust::cxxqtlib1::qjsonobjectLen;
  return qjsonobject_len$(object);
}

void rust$cxxqtlib1$cxxbridge1$205$qjsonobject_insert(::QJsonObject &object, ::QString const &key, ::QJsonValue const &value) noexcept {
  void (*qjsonobject_insert$)(::QJsonObject &, ::QString const &, ::QJsonValue const &) = ::rust::cxxqtlib1::qjsonobjectInsert;
  qjsonobject_insert$(object, key, value);
}
} // extern "C"
} // namespace cxxqtlib1
} // namespace rust
