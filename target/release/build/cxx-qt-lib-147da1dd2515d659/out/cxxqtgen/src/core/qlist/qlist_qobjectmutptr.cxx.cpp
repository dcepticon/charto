#include "cxx-qt-lib/qobjectmutptr.h"
#include "cxx-qt-lib/core/qlist/qlist_QObjectMutPtr.h"
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
    ::rust::IsRelocatable<::QList_QObjectMutPtr>::value,
    "type QList_QObjectMutPtr should be trivially move constructible and trivially destructible in C++ to be used as a return value of `qlist_clone_QObjectMutPtr`, `qlist_default_QObjectMutPtr` in Rust");

extern "C" {
void cxxbridge1$205$QList_QObjectMutPtr$cxx_clear(::QList_QObjectMutPtr &self) noexcept {
  void (::QList_QObjectMutPtr::*cxx_clear$)() = &::QList_QObjectMutPtr::clear;
  (self.*cxx_clear$)();
}

bool cxxbridge1$205$QList_QObjectMutPtr$cxx_contains(::QList_QObjectMutPtr const &self, ::QObjectMutPtr const &arg0) noexcept {
  bool (::QList_QObjectMutPtr::*cxx_contains$)(::QObjectMutPtr const &) const = &::QList_QObjectMutPtr::contains;
  return (self.*cxx_contains$)(arg0);
}
} // extern "C"

namespace rust {
namespace cxxqtlib1 {
extern "C" {
void rust$cxxqtlib1$cxxbridge1$205$qlist_clone_QObjectMutPtr(::QList_QObjectMutPtr const &arg0, ::QList_QObjectMutPtr *return$) noexcept {
  ::QList_QObjectMutPtr (*qlist_clone_QObjectMutPtr$)(::QList_QObjectMutPtr const &) = ::rust::cxxqtlib1::construct;
  new (return$) ::QList_QObjectMutPtr(qlist_clone_QObjectMutPtr$(arg0));
}

void rust$cxxqtlib1$cxxbridge1$205$qlist_default_QObjectMutPtr(::QList_QObjectMutPtr *return$) noexcept {
  ::QList_QObjectMutPtr (*qlist_default_QObjectMutPtr$)() = ::rust::cxxqtlib1::construct;
  new (return$) ::QList_QObjectMutPtr(qlist_default_QObjectMutPtr$());
}

void rust$cxxqtlib1$cxxbridge1$205$qlist_drop_QObjectMutPtr(::QList_QObjectMutPtr &arg0) noexcept {
  void (*qlist_drop_QObjectMutPtr$)(::QList_QObjectMutPtr &) = ::rust::cxxqtlib1::drop;
  qlist_drop_QObjectMutPtr$(arg0);
}
} // extern "C"

namespace qlist {
extern "C" {
void rust$cxxqtlib1$qlist$cxxbridge1$205$reserve_QObjectMutPtr(::QList_QObjectMutPtr &arg0, ::rust::isize size) noexcept {
  void (*reserve_QObjectMutPtr$)(::QList_QObjectMutPtr &, ::rust::isize) = ::rust::cxxqtlib1::qlist::qlistReserve;
  reserve_QObjectMutPtr$(arg0, size);
}

void rust$cxxqtlib1$qlist$cxxbridge1$205$append_QObjectMutPtr(::QList_QObjectMutPtr &arg0, ::QObjectMutPtr const &arg1) noexcept {
  void (*append_QObjectMutPtr$)(::QList_QObjectMutPtr &, ::QObjectMutPtr const &) = ::rust::cxxqtlib1::qlist::qlistAppend;
  append_QObjectMutPtr$(arg0, arg1);
}

void rust$cxxqtlib1$qlist$cxxbridge1$205$get_unchecked_QObjectMutPtr(::QList_QObjectMutPtr const &set, ::rust::isize pos, ::QObjectMutPtr const **return$) noexcept {
  ::QObjectMutPtr const &(*get_unchecked_QObjectMutPtr$)(::QList_QObjectMutPtr const &, ::rust::isize) = ::rust::cxxqtlib1::qlist::qlistGetUnchecked;
  new (return$) ::QObjectMutPtr const *(&get_unchecked_QObjectMutPtr$(set, pos));
}

::rust::isize rust$cxxqtlib1$qlist$cxxbridge1$205$index_of_QObjectMutPtr(::QList_QObjectMutPtr const &arg0, ::QObjectMutPtr const &arg1) noexcept {
  ::rust::isize (*index_of_QObjectMutPtr$)(::QList_QObjectMutPtr const &, ::QObjectMutPtr const &) = ::rust::cxxqtlib1::qlist::qlistIndexOf;
  return index_of_QObjectMutPtr$(arg0, arg1);
}

void rust$cxxqtlib1$qlist$cxxbridge1$205$insert_QObjectMutPtr(::QList_QObjectMutPtr &arg0, ::rust::isize arg1, ::QObjectMutPtr const &arg2) noexcept {
  void (*insert_QObjectMutPtr$)(::QList_QObjectMutPtr &, ::rust::isize, ::QObjectMutPtr const &) = ::rust::cxxqtlib1::qlist::qlistInsert;
  insert_QObjectMutPtr$(arg0, arg1, arg2);
}

void rust$cxxqtlib1$qlist$cxxbridge1$205$remove_QObjectMutPtr(::QList_QObjectMutPtr &arg0, ::rust::isize arg1) noexcept {
  void (*remove_QObjectMutPtr$)(::QList_QObjectMutPtr &, ::rust::isize) = ::rust::cxxqtlib1::qlist::qlistRemove;
  remove_QObjectMutPtr$(arg0, arg1);
}

::rust::isize rust$cxxqtlib1$qlist$cxxbridge1$205$len_QObjectMutPtr(::QList_QObjectMutPtr const &arg0) noexcept {
  ::rust::isize (*len_QObjectMutPtr$)(::QList_QObjectMutPtr const &) = ::rust::cxxqtlib1::qlist::qlistLen;
  return len_QObjectMutPtr$(arg0);
}
} // extern "C"
} // namespace qlist
} // namespace cxxqtlib1
} // namespace rust
