#include "charto-app/src/backend.cxxqt.h"


// Define namespace otherwise we hit a GCC bug
// https://gcc.gnu.org/bugzilla/show_bug.cgi?id=56480
namespace rust::cxxqt1 {
template <>
SignalHandler<::rust::cxxqtgen1::ChartBackendCxxQtSignalParamssnapshotChanged *>::~SignalHandler() noexcept
{
    if (data[0] == nullptr && data[1] == nullptr)
    {
        return;
    }

    drop_ChartBackend_signal_handler_snapshotChanged(::std::move(*this));
}

template <>
template <>
void SignalHandler<::rust::cxxqtgen1::ChartBackendCxxQtSignalParamssnapshotChanged *>::operator()<ChartBackend&>(ChartBackend& self)
{
    call_ChartBackend_signal_handler_snapshotChanged(*this, self);
}

static_assert(alignof(SignalHandler<::rust::cxxqtgen1::ChartBackendCxxQtSignalParamssnapshotChanged *>) <= alignof(::std::size_t), "unexpected aligment");
static_assert(sizeof(SignalHandler<::rust::cxxqtgen1::ChartBackendCxxQtSignalParamssnapshotChanged *>) == sizeof(::std::size_t[2]), "unexpected size");
} // namespace rust::cxxqt1

namespace rust::cxxqtgen1 {
::QMetaObject::Connection
ChartBackend_snapshotChangedConnect(ChartBackend& self, ::rust::cxxqtgen1::ChartBackendCxxQtSignalHandlersnapshotChanged closure, ::Qt::ConnectionType type)
{
    return ::QObject::connect(
        &self,
        &ChartBackend::snapshotChanged,
        &self,
        [&, closure = ::std::move(closure)]() mutable {
            closure.template operator()<ChartBackend&>(self);
        },
        type);
}
} // namespace rust::cxxqtgen1

// Define namespace otherwise we hit a GCC bug
// https://gcc.gnu.org/bugzilla/show_bug.cgi?id=56480
namespace rust::cxxqt1 {
template <>
SignalHandler<::rust::cxxqtgen1::ChartBackendCxxQtSignalParamsstatusChanged *>::~SignalHandler() noexcept
{
    if (data[0] == nullptr && data[1] == nullptr)
    {
        return;
    }

    drop_ChartBackend_signal_handler_statusChanged(::std::move(*this));
}

template <>
template <>
void SignalHandler<::rust::cxxqtgen1::ChartBackendCxxQtSignalParamsstatusChanged *>::operator()<ChartBackend&>(ChartBackend& self)
{
    call_ChartBackend_signal_handler_statusChanged(*this, self);
}

static_assert(alignof(SignalHandler<::rust::cxxqtgen1::ChartBackendCxxQtSignalParamsstatusChanged *>) <= alignof(::std::size_t), "unexpected aligment");
static_assert(sizeof(SignalHandler<::rust::cxxqtgen1::ChartBackendCxxQtSignalParamsstatusChanged *>) == sizeof(::std::size_t[2]), "unexpected size");
} // namespace rust::cxxqt1

namespace rust::cxxqtgen1 {
::QMetaObject::Connection
ChartBackend_statusChangedConnect(ChartBackend& self, ::rust::cxxqtgen1::ChartBackendCxxQtSignalHandlerstatusChanged closure, ::Qt::ConnectionType type)
{
    return ::QObject::connect(
        &self,
        &ChartBackend::statusChanged,
        &self,
        [&, closure = ::std::move(closure)]() mutable {
            closure.template operator()<ChartBackend&>(self);
        },
        type);
}
} // namespace rust::cxxqtgen1

// Define namespace otherwise we hit a GCC bug
// https://gcc.gnu.org/bugzilla/show_bug.cgi?id=56480
namespace rust::cxxqt1 {
template <>
SignalHandler<::rust::cxxqtgen1::ChartBackendCxxQtSignalParamspickedChanged *>::~SignalHandler() noexcept
{
    if (data[0] == nullptr && data[1] == nullptr)
    {
        return;
    }

    drop_ChartBackend_signal_handler_pickedChanged(::std::move(*this));
}

template <>
template <>
void SignalHandler<::rust::cxxqtgen1::ChartBackendCxxQtSignalParamspickedChanged *>::operator()<ChartBackend&>(ChartBackend& self)
{
    call_ChartBackend_signal_handler_pickedChanged(*this, self);
}

static_assert(alignof(SignalHandler<::rust::cxxqtgen1::ChartBackendCxxQtSignalParamspickedChanged *>) <= alignof(::std::size_t), "unexpected aligment");
static_assert(sizeof(SignalHandler<::rust::cxxqtgen1::ChartBackendCxxQtSignalParamspickedChanged *>) == sizeof(::std::size_t[2]), "unexpected size");
} // namespace rust::cxxqt1

namespace rust::cxxqtgen1 {
::QMetaObject::Connection
ChartBackend_pickedChangedConnect(ChartBackend& self, ::rust::cxxqtgen1::ChartBackendCxxQtSignalHandlerpickedChanged closure, ::Qt::ConnectionType type)
{
    return ::QObject::connect(
        &self,
        &ChartBackend::pickedChanged,
        &self,
        [&, closure = ::std::move(closure)]() mutable {
            closure.template operator()<ChartBackend&>(self);
        },
        type);
}
} // namespace rust::cxxqtgen1

static_assert(alignof(ChartBackendCxxQtThread) <= alignof(::std::size_t), "unexpected aligment");
static_assert(sizeof(ChartBackendCxxQtThread) == sizeof(::std::size_t[2]), "unexpected size");

ChartBackend::ChartBackend(QObject* parent)
  : QObject(parent)
  , ::rust::cxxqt1::CxxQtType<ChartBackendRust>(::cxx_qt_ChartBackend::createRs())
  , ::rust::cxxqt1::CxxQtThreading<ChartBackend>(this)
{ }

