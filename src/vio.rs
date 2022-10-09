pub trait Vio: Default {
    const VIRTIO_IRQ: u64;
    const VRING_DESC_SIZE: u64;
    const QUEUE_SIZE: u64;
    const SECTOR_SIZE: u64;
    const VIRTQ_DESC_F_NEXT: u64;
    const VIRTQ_DESC_F_WRITE: u64;
    const _VIRTQ_DESC_F_INDIRECT: u64;
    const MAGIC_START: u64;
    const MAGIC_END: u64;
    const VERSION_START: u64;
    const VERSION_END: u64;
    const DEVICE_ID_START: u64;
    const DEVICE_ID_END: u64;
    const VENDOR_ID_START: u64;
    const VENDER_ID_END: u64;
    const DEVICES_FEATURES_START: u64;
    const DEVICES_FEATURES_END: u64;
    const DEVICE_FEATURES_SELECTION_START: u64;
    const DEVICE_FEATURES_SELF_END: u64;
    const DRIVER_FEATURES_START: u64;
    const DRIVER_FEATURES_END: u64;
    const DRIVER_FEATURES_SELECTION_START: u64;
    const DRIVER_FEATURES_SELECTION_END: u64;
    const GUEST_PAGE_SIZE_START: u64;
    const GUEST_PAGE_SIZE_END: u64;
    const QUEUE_SELECTION_START: u64;
    const QUEUE_SELECTION_END: u64;
    const QUEUE_NUM_MAX_START: u64;
    const QUEUE_NUM_MAX_END: u64;
    const QUEUE_ALIGN_START: u64;
    const QUEUE_ALIGN_END: u64;
    const QUEUE_PFN_START: u64;
    const QUEUE_PFN_END: u64;
    const QUEUE_NOTIFY_START: u64;
    const QUEUE_NOTIFY_END: u64;
    const INTERRUPT_STATUS_START: u64;
    const INTERRUPT_STATUS_END: u64;
    const INTERRUPT_ACKNOWLEDGEMENT_START: u64;
    const INTERRUPT_ACKNOWLEDGEMENT_END: u64;
    const STATUS_START: u64;
    const STATUS_END: u64;
    const CONFIG_START: u64;
    const CONFIG_END: u64;
    pub type VirtualQueueAddress;
    pub type VirtualQueueDescriptor;
    pub type VirtualQueueAvailability;
    pub type Exception;
    pub type Cpu;

    fn new_virtual_queue_address(self) -> Self::VirtualQueueAddress;
    fn new_virtual_queue_descriptor(self) -> Self::VirtualQueueDescriptor;
    fn new_virtual_queue_availability(self, cpu: Self::Cpu) -> Result<Self::VirtualQueueAvailability, Self::Exception>;
    fn new() -> Self {
        Self::default()
    }; 


}
