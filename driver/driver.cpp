/*
 * Biquad Studio Virtual Audio Driver
 * Based on Microsoft Sysvad WDK Sample
 *
 * This file contains the DriverEntry and WDF/PortCls initialization.
 */

#include <portcls.h>
#include <stddef.h>

#pragma code_seg("INIT")

extern "C"
NTSTATUS
DriverEntry(
    _In_  PDRIVER_OBJECT  DriverObject,
    _In_  PUNICODE_STRING RegistryPath
)
{
    NTSTATUS status;

    // Initialize PortCls (Audio Class Extension)
    status = PcInitializeAdapterDriver(
        DriverObject,
        RegistryPath,
        (PDRIVER_ADD_DEVICE)AddDevice
    );

    return status;
}

#pragma code_seg("PAGE")

NTSTATUS
AddDevice(
    _In_  PDRIVER_OBJECT  DriverObject,
    _In_  PDEVICE_OBJECT  PhysicalDeviceObject
)
{
    PAGED_CODE();
    NTSTATUS status = STATUS_SUCCESS;

    // Bind PortCls to the new device object
    status = PcAddAdapterDevice(
        DriverObject,
        PhysicalDeviceObject,
        StartDevice,
        MAX_MINIPORTS,
        0
    );

    return status;
}

NTSTATUS
StartDevice(
    _In_  PDEVICE_OBJECT  DeviceObject,
    _In_  PIRP            Irp,
    _In_  PRESOURCELIST   ResourceList
)
{
    PAGED_CODE();
    NTSTATUS status = STATUS_SUCCESS;

    // TODO: Register MiniportWaveRT endpoints for Game, Chat, Media, Mic
    // e.g., PcRegisterSubdevice(DeviceObject, L"Game", Port, Miniport);
    
    return status;
}
