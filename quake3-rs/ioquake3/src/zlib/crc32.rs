use ::libc;

pub mod crc32_h {
    /* crc32.h -- tables for rapid CRC calculation
     * Generated automatically by crc32.c
     */

    pub static mut crc_table: [[libc::c_ulong; 256]; 8] = [
        [
            0 as libc::c_ulong,
            0x77073096 as libc::c_ulong,
            0xee0e612c as libc::c_ulong,
            0x990951ba as libc::c_ulong,
            0x76dc419 as libc::c_ulong,
            0x706af48f as libc::c_ulong,
            0xe963a535 as libc::c_ulong,
            0x9e6495a3 as libc::c_ulong,
            0xedb8832 as libc::c_ulong,
            0x79dcb8a4 as libc::c_ulong,
            0xe0d5e91e as libc::c_ulong,
            0x97d2d988 as libc::c_ulong,
            0x9b64c2b as libc::c_ulong,
            0x7eb17cbd as libc::c_ulong,
            0xe7b82d07 as libc::c_ulong,
            0x90bf1d91 as libc::c_ulong,
            0x1db71064 as libc::c_ulong,
            0x6ab020f2 as libc::c_ulong,
            0xf3b97148 as libc::c_ulong,
            0x84be41de as libc::c_ulong,
            0x1adad47d as libc::c_ulong,
            0x6ddde4eb as libc::c_ulong,
            0xf4d4b551 as libc::c_ulong,
            0x83d385c7 as libc::c_ulong,
            0x136c9856 as libc::c_ulong,
            0x646ba8c0 as libc::c_ulong,
            0xfd62f97a as libc::c_ulong,
            0x8a65c9ec as libc::c_ulong,
            0x14015c4f as libc::c_ulong,
            0x63066cd9 as libc::c_ulong,
            0xfa0f3d63 as libc::c_ulong,
            0x8d080df5 as libc::c_ulong,
            0x3b6e20c8 as libc::c_ulong,
            0x4c69105e as libc::c_ulong,
            0xd56041e4 as libc::c_ulong,
            0xa2677172 as libc::c_ulong,
            0x3c03e4d1 as libc::c_ulong,
            0x4b04d447 as libc::c_ulong,
            0xd20d85fd as libc::c_ulong,
            0xa50ab56b as libc::c_ulong,
            0x35b5a8fa as libc::c_ulong,
            0x42b2986c as libc::c_ulong,
            0xdbbbc9d6 as libc::c_ulong,
            0xacbcf940 as libc::c_ulong,
            0x32d86ce3 as libc::c_ulong,
            0x45df5c75 as libc::c_ulong,
            0xdcd60dcf as libc::c_ulong,
            0xabd13d59 as libc::c_ulong,
            0x26d930ac as libc::c_ulong,
            0x51de003a as libc::c_ulong,
            0xc8d75180 as libc::c_ulong,
            0xbfd06116 as libc::c_ulong,
            0x21b4f4b5 as libc::c_ulong,
            0x56b3c423 as libc::c_ulong,
            0xcfba9599 as libc::c_ulong,
            0xb8bda50f as libc::c_ulong,
            0x2802b89e as libc::c_ulong,
            0x5f058808 as libc::c_ulong,
            0xc60cd9b2 as libc::c_ulong,
            0xb10be924 as libc::c_ulong,
            0x2f6f7c87 as libc::c_ulong,
            0x58684c11 as libc::c_ulong,
            0xc1611dab as libc::c_ulong,
            0xb6662d3d as libc::c_ulong,
            0x76dc4190 as libc::c_ulong,
            0x1db7106 as libc::c_ulong,
            0x98d220bc as libc::c_ulong,
            0xefd5102a as libc::c_ulong,
            0x71b18589 as libc::c_ulong,
            0x6b6b51f as libc::c_ulong,
            0x9fbfe4a5 as libc::c_ulong,
            0xe8b8d433 as libc::c_ulong,
            0x7807c9a2 as libc::c_ulong,
            0xf00f934 as libc::c_ulong,
            0x9609a88e as libc::c_ulong,
            0xe10e9818 as libc::c_ulong,
            0x7f6a0dbb as libc::c_ulong,
            0x86d3d2d as libc::c_ulong,
            0x91646c97 as libc::c_ulong,
            0xe6635c01 as libc::c_ulong,
            0x6b6b51f4 as libc::c_ulong,
            0x1c6c6162 as libc::c_ulong,
            0x856530d8 as libc::c_ulong,
            0xf262004e as libc::c_ulong,
            0x6c0695ed as libc::c_ulong,
            0x1b01a57b as libc::c_ulong,
            0x8208f4c1 as libc::c_ulong,
            0xf50fc457 as libc::c_ulong,
            0x65b0d9c6 as libc::c_ulong,
            0x12b7e950 as libc::c_ulong,
            0x8bbeb8ea as libc::c_ulong,
            0xfcb9887c as libc::c_ulong,
            0x62dd1ddf as libc::c_ulong,
            0x15da2d49 as libc::c_ulong,
            0x8cd37cf3 as libc::c_ulong,
            0xfbd44c65 as libc::c_ulong,
            0x4db26158 as libc::c_ulong,
            0x3ab551ce as libc::c_ulong,
            0xa3bc0074 as libc::c_ulong,
            0xd4bb30e2 as libc::c_ulong,
            0x4adfa541 as libc::c_ulong,
            0x3dd895d7 as libc::c_ulong,
            0xa4d1c46d as libc::c_ulong,
            0xd3d6f4fb as libc::c_ulong,
            0x4369e96a as libc::c_ulong,
            0x346ed9fc as libc::c_ulong,
            0xad678846 as libc::c_ulong,
            0xda60b8d0 as libc::c_ulong,
            0x44042d73 as libc::c_ulong,
            0x33031de5 as libc::c_ulong,
            0xaa0a4c5f as libc::c_ulong,
            0xdd0d7cc9 as libc::c_ulong,
            0x5005713c as libc::c_ulong,
            0x270241aa as libc::c_ulong,
            0xbe0b1010 as libc::c_ulong,
            0xc90c2086 as libc::c_ulong,
            0x5768b525 as libc::c_ulong,
            0x206f85b3 as libc::c_ulong,
            0xb966d409 as libc::c_ulong,
            0xce61e49f as libc::c_ulong,
            0x5edef90e as libc::c_ulong,
            0x29d9c998 as libc::c_ulong,
            0xb0d09822 as libc::c_ulong,
            0xc7d7a8b4 as libc::c_ulong,
            0x59b33d17 as libc::c_ulong,
            0x2eb40d81 as libc::c_ulong,
            0xb7bd5c3b as libc::c_ulong,
            0xc0ba6cad as libc::c_ulong,
            0xedb88320 as libc::c_ulong,
            0x9abfb3b6 as libc::c_ulong,
            0x3b6e20c as libc::c_ulong,
            0x74b1d29a as libc::c_ulong,
            0xead54739 as libc::c_ulong,
            0x9dd277af as libc::c_ulong,
            0x4db2615 as libc::c_ulong,
            0x73dc1683 as libc::c_ulong,
            0xe3630b12 as libc::c_ulong,
            0x94643b84 as libc::c_ulong,
            0xd6d6a3e as libc::c_ulong,
            0x7a6a5aa8 as libc::c_ulong,
            0xe40ecf0b as libc::c_ulong,
            0x9309ff9d as libc::c_ulong,
            0xa00ae27 as libc::c_ulong,
            0x7d079eb1 as libc::c_ulong,
            0xf00f9344 as libc::c_ulong,
            0x8708a3d2 as libc::c_ulong,
            0x1e01f268 as libc::c_ulong,
            0x6906c2fe as libc::c_ulong,
            0xf762575d as libc::c_ulong,
            0x806567cb as libc::c_ulong,
            0x196c3671 as libc::c_ulong,
            0x6e6b06e7 as libc::c_ulong,
            0xfed41b76 as libc::c_ulong,
            0x89d32be0 as libc::c_ulong,
            0x10da7a5a as libc::c_ulong,
            0x67dd4acc as libc::c_ulong,
            0xf9b9df6f as libc::c_ulong,
            0x8ebeeff9 as libc::c_ulong,
            0x17b7be43 as libc::c_ulong,
            0x60b08ed5 as libc::c_ulong,
            0xd6d6a3e8 as libc::c_ulong,
            0xa1d1937e as libc::c_ulong,
            0x38d8c2c4 as libc::c_ulong,
            0x4fdff252 as libc::c_ulong,
            0xd1bb67f1 as libc::c_ulong,
            0xa6bc5767 as libc::c_ulong,
            0x3fb506dd as libc::c_ulong,
            0x48b2364b as libc::c_ulong,
            0xd80d2bda as libc::c_ulong,
            0xaf0a1b4c as libc::c_ulong,
            0x36034af6 as libc::c_ulong,
            0x41047a60 as libc::c_ulong,
            0xdf60efc3 as libc::c_ulong,
            0xa867df55 as libc::c_ulong,
            0x316e8eef as libc::c_ulong,
            0x4669be79 as libc::c_ulong,
            0xcb61b38c as libc::c_ulong,
            0xbc66831a as libc::c_ulong,
            0x256fd2a0 as libc::c_ulong,
            0x5268e236 as libc::c_ulong,
            0xcc0c7795 as libc::c_ulong,
            0xbb0b4703 as libc::c_ulong,
            0x220216b9 as libc::c_ulong,
            0x5505262f as libc::c_ulong,
            0xc5ba3bbe as libc::c_ulong,
            0xb2bd0b28 as libc::c_ulong,
            0x2bb45a92 as libc::c_ulong,
            0x5cb36a04 as libc::c_ulong,
            0xc2d7ffa7 as libc::c_ulong,
            0xb5d0cf31 as libc::c_ulong,
            0x2cd99e8b as libc::c_ulong,
            0x5bdeae1d as libc::c_ulong,
            0x9b64c2b0 as libc::c_ulong,
            0xec63f226 as libc::c_ulong,
            0x756aa39c as libc::c_ulong,
            0x26d930a as libc::c_ulong,
            0x9c0906a9 as libc::c_ulong,
            0xeb0e363f as libc::c_ulong,
            0x72076785 as libc::c_ulong,
            0x5005713 as libc::c_ulong,
            0x95bf4a82 as libc::c_ulong,
            0xe2b87a14 as libc::c_ulong,
            0x7bb12bae as libc::c_ulong,
            0xcb61b38 as libc::c_ulong,
            0x92d28e9b as libc::c_ulong,
            0xe5d5be0d as libc::c_ulong,
            0x7cdcefb7 as libc::c_ulong,
            0xbdbdf21 as libc::c_ulong,
            0x86d3d2d4 as libc::c_ulong,
            0xf1d4e242 as libc::c_ulong,
            0x68ddb3f8 as libc::c_ulong,
            0x1fda836e as libc::c_ulong,
            0x81be16cd as libc::c_ulong,
            0xf6b9265b as libc::c_ulong,
            0x6fb077e1 as libc::c_ulong,
            0x18b74777 as libc::c_ulong,
            0x88085ae6 as libc::c_ulong,
            0xff0f6a70 as libc::c_ulong,
            0x66063bca as libc::c_ulong,
            0x11010b5c as libc::c_ulong,
            0x8f659eff as libc::c_ulong,
            0xf862ae69 as libc::c_ulong,
            0x616bffd3 as libc::c_ulong,
            0x166ccf45 as libc::c_ulong,
            0xa00ae278 as libc::c_ulong,
            0xd70dd2ee as libc::c_ulong,
            0x4e048354 as libc::c_ulong,
            0x3903b3c2 as libc::c_ulong,
            0xa7672661 as libc::c_ulong,
            0xd06016f7 as libc::c_ulong,
            0x4969474d as libc::c_ulong,
            0x3e6e77db as libc::c_ulong,
            0xaed16a4a as libc::c_ulong,
            0xd9d65adc as libc::c_ulong,
            0x40df0b66 as libc::c_ulong,
            0x37d83bf0 as libc::c_ulong,
            0xa9bcae53 as libc::c_ulong,
            0xdebb9ec5 as libc::c_ulong,
            0x47b2cf7f as libc::c_ulong,
            0x30b5ffe9 as libc::c_ulong,
            0xbdbdf21c as libc::c_ulong,
            0xcabac28a as libc::c_ulong,
            0x53b39330 as libc::c_ulong,
            0x24b4a3a6 as libc::c_ulong,
            0xbad03605 as libc::c_ulong,
            0xcdd70693 as libc::c_ulong,
            0x54de5729 as libc::c_ulong,
            0x23d967bf as libc::c_ulong,
            0xb3667a2e as libc::c_ulong,
            0xc4614ab8 as libc::c_ulong,
            0x5d681b02 as libc::c_ulong,
            0x2a6f2b94 as libc::c_ulong,
            0xb40bbe37 as libc::c_ulong,
            0xc30c8ea1 as libc::c_ulong,
            0x5a05df1b as libc::c_ulong,
            0x2d02ef8d as libc::c_ulong,
        ],
        [
            0 as libc::c_ulong,
            0x191b3141 as libc::c_ulong,
            0x32366282 as libc::c_ulong,
            0x2b2d53c3 as libc::c_ulong,
            0x646cc504 as libc::c_ulong,
            0x7d77f445 as libc::c_ulong,
            0x565aa786 as libc::c_ulong,
            0x4f4196c7 as libc::c_ulong,
            0xc8d98a08 as libc::c_ulong,
            0xd1c2bb49 as libc::c_ulong,
            0xfaefe88a as libc::c_ulong,
            0xe3f4d9cb as libc::c_ulong,
            0xacb54f0c as libc::c_ulong,
            0xb5ae7e4d as libc::c_ulong,
            0x9e832d8e as libc::c_ulong,
            0x87981ccf as libc::c_ulong,
            0x4ac21251 as libc::c_ulong,
            0x53d92310 as libc::c_ulong,
            0x78f470d3 as libc::c_ulong,
            0x61ef4192 as libc::c_ulong,
            0x2eaed755 as libc::c_ulong,
            0x37b5e614 as libc::c_ulong,
            0x1c98b5d7 as libc::c_ulong,
            0x5838496 as libc::c_ulong,
            0x821b9859 as libc::c_ulong,
            0x9b00a918 as libc::c_ulong,
            0xb02dfadb as libc::c_ulong,
            0xa936cb9a as libc::c_ulong,
            0xe6775d5d as libc::c_ulong,
            0xff6c6c1c as libc::c_ulong,
            0xd4413fdf as libc::c_ulong,
            0xcd5a0e9e as libc::c_ulong,
            0x958424a2 as libc::c_ulong,
            0x8c9f15e3 as libc::c_ulong,
            0xa7b24620 as libc::c_ulong,
            0xbea97761 as libc::c_ulong,
            0xf1e8e1a6 as libc::c_ulong,
            0xe8f3d0e7 as libc::c_ulong,
            0xc3de8324 as libc::c_ulong,
            0xdac5b265 as libc::c_ulong,
            0x5d5daeaa as libc::c_ulong,
            0x44469feb as libc::c_ulong,
            0x6f6bcc28 as libc::c_ulong,
            0x7670fd69 as libc::c_ulong,
            0x39316bae as libc::c_ulong,
            0x202a5aef as libc::c_ulong,
            0xb07092c as libc::c_ulong,
            0x121c386d as libc::c_ulong,
            0xdf4636f3 as libc::c_ulong,
            0xc65d07b2 as libc::c_ulong,
            0xed705471 as libc::c_ulong,
            0xf46b6530 as libc::c_ulong,
            0xbb2af3f7 as libc::c_ulong,
            0xa231c2b6 as libc::c_ulong,
            0x891c9175 as libc::c_ulong,
            0x9007a034 as libc::c_ulong,
            0x179fbcfb as libc::c_ulong,
            0xe848dba as libc::c_ulong,
            0x25a9de79 as libc::c_ulong,
            0x3cb2ef38 as libc::c_ulong,
            0x73f379ff as libc::c_ulong,
            0x6ae848be as libc::c_ulong,
            0x41c51b7d as libc::c_ulong,
            0x58de2a3c as libc::c_ulong,
            0xf0794f05 as libc::c_ulong,
            0xe9627e44 as libc::c_ulong,
            0xc24f2d87 as libc::c_ulong,
            0xdb541cc6 as libc::c_ulong,
            0x94158a01 as libc::c_ulong,
            0x8d0ebb40 as libc::c_ulong,
            0xa623e883 as libc::c_ulong,
            0xbf38d9c2 as libc::c_ulong,
            0x38a0c50d as libc::c_ulong,
            0x21bbf44c as libc::c_ulong,
            0xa96a78f as libc::c_ulong,
            0x138d96ce as libc::c_ulong,
            0x5ccc0009 as libc::c_ulong,
            0x45d73148 as libc::c_ulong,
            0x6efa628b as libc::c_ulong,
            0x77e153ca as libc::c_ulong,
            0xbabb5d54 as libc::c_ulong,
            0xa3a06c15 as libc::c_ulong,
            0x888d3fd6 as libc::c_ulong,
            0x91960e97 as libc::c_ulong,
            0xded79850 as libc::c_ulong,
            0xc7cca911 as libc::c_ulong,
            0xece1fad2 as libc::c_ulong,
            0xf5facb93 as libc::c_ulong,
            0x7262d75c as libc::c_ulong,
            0x6b79e61d as libc::c_ulong,
            0x4054b5de as libc::c_ulong,
            0x594f849f as libc::c_ulong,
            0x160e1258 as libc::c_ulong,
            0xf152319 as libc::c_ulong,
            0x243870da as libc::c_ulong,
            0x3d23419b as libc::c_ulong,
            0x65fd6ba7 as libc::c_ulong,
            0x7ce65ae6 as libc::c_ulong,
            0x57cb0925 as libc::c_ulong,
            0x4ed03864 as libc::c_ulong,
            0x191aea3 as libc::c_ulong,
            0x188a9fe2 as libc::c_ulong,
            0x33a7cc21 as libc::c_ulong,
            0x2abcfd60 as libc::c_ulong,
            0xad24e1af as libc::c_ulong,
            0xb43fd0ee as libc::c_ulong,
            0x9f12832d as libc::c_ulong,
            0x8609b26c as libc::c_ulong,
            0xc94824ab as libc::c_ulong,
            0xd05315ea as libc::c_ulong,
            0xfb7e4629 as libc::c_ulong,
            0xe2657768 as libc::c_ulong,
            0x2f3f79f6 as libc::c_ulong,
            0x362448b7 as libc::c_ulong,
            0x1d091b74 as libc::c_ulong,
            0x4122a35 as libc::c_ulong,
            0x4b53bcf2 as libc::c_ulong,
            0x52488db3 as libc::c_ulong,
            0x7965de70 as libc::c_ulong,
            0x607eef31 as libc::c_ulong,
            0xe7e6f3fe as libc::c_ulong,
            0xfefdc2bf as libc::c_ulong,
            0xd5d0917c as libc::c_ulong,
            0xcccba03d as libc::c_ulong,
            0x838a36fa as libc::c_ulong,
            0x9a9107bb as libc::c_ulong,
            0xb1bc5478 as libc::c_ulong,
            0xa8a76539 as libc::c_ulong,
            0x3b83984b as libc::c_ulong,
            0x2298a90a as libc::c_ulong,
            0x9b5fac9 as libc::c_ulong,
            0x10aecb88 as libc::c_ulong,
            0x5fef5d4f as libc::c_ulong,
            0x46f46c0e as libc::c_ulong,
            0x6dd93fcd as libc::c_ulong,
            0x74c20e8c as libc::c_ulong,
            0xf35a1243 as libc::c_ulong,
            0xea412302 as libc::c_ulong,
            0xc16c70c1 as libc::c_ulong,
            0xd8774180 as libc::c_ulong,
            0x9736d747 as libc::c_ulong,
            0x8e2de606 as libc::c_ulong,
            0xa500b5c5 as libc::c_ulong,
            0xbc1b8484 as libc::c_ulong,
            0x71418a1a as libc::c_ulong,
            0x685abb5b as libc::c_ulong,
            0x4377e898 as libc::c_ulong,
            0x5a6cd9d9 as libc::c_ulong,
            0x152d4f1e as libc::c_ulong,
            0xc367e5f as libc::c_ulong,
            0x271b2d9c as libc::c_ulong,
            0x3e001cdd as libc::c_ulong,
            0xb9980012 as libc::c_ulong,
            0xa0833153 as libc::c_ulong,
            0x8bae6290 as libc::c_ulong,
            0x92b553d1 as libc::c_ulong,
            0xddf4c516 as libc::c_ulong,
            0xc4eff457 as libc::c_ulong,
            0xefc2a794 as libc::c_ulong,
            0xf6d996d5 as libc::c_ulong,
            0xae07bce9 as libc::c_ulong,
            0xb71c8da8 as libc::c_ulong,
            0x9c31de6b as libc::c_ulong,
            0x852aef2a as libc::c_ulong,
            0xca6b79ed as libc::c_ulong,
            0xd37048ac as libc::c_ulong,
            0xf85d1b6f as libc::c_ulong,
            0xe1462a2e as libc::c_ulong,
            0x66de36e1 as libc::c_ulong,
            0x7fc507a0 as libc::c_ulong,
            0x54e85463 as libc::c_ulong,
            0x4df36522 as libc::c_ulong,
            0x2b2f3e5 as libc::c_ulong,
            0x1ba9c2a4 as libc::c_ulong,
            0x30849167 as libc::c_ulong,
            0x299fa026 as libc::c_ulong,
            0xe4c5aeb8 as libc::c_ulong,
            0xfdde9ff9 as libc::c_ulong,
            0xd6f3cc3a as libc::c_ulong,
            0xcfe8fd7b as libc::c_ulong,
            0x80a96bbc as libc::c_ulong,
            0x99b25afd as libc::c_ulong,
            0xb29f093e as libc::c_ulong,
            0xab84387f as libc::c_ulong,
            0x2c1c24b0 as libc::c_ulong,
            0x350715f1 as libc::c_ulong,
            0x1e2a4632 as libc::c_ulong,
            0x7317773 as libc::c_ulong,
            0x4870e1b4 as libc::c_ulong,
            0x516bd0f5 as libc::c_ulong,
            0x7a468336 as libc::c_ulong,
            0x635db277 as libc::c_ulong,
            0xcbfad74e as libc::c_ulong,
            0xd2e1e60f as libc::c_ulong,
            0xf9ccb5cc as libc::c_ulong,
            0xe0d7848d as libc::c_ulong,
            0xaf96124a as libc::c_ulong,
            0xb68d230b as libc::c_ulong,
            0x9da070c8 as libc::c_ulong,
            0x84bb4189 as libc::c_ulong,
            0x3235d46 as libc::c_ulong,
            0x1a386c07 as libc::c_ulong,
            0x31153fc4 as libc::c_ulong,
            0x280e0e85 as libc::c_ulong,
            0x674f9842 as libc::c_ulong,
            0x7e54a903 as libc::c_ulong,
            0x5579fac0 as libc::c_ulong,
            0x4c62cb81 as libc::c_ulong,
            0x8138c51f as libc::c_ulong,
            0x9823f45e as libc::c_ulong,
            0xb30ea79d as libc::c_ulong,
            0xaa1596dc as libc::c_ulong,
            0xe554001b as libc::c_ulong,
            0xfc4f315a as libc::c_ulong,
            0xd7626299 as libc::c_ulong,
            0xce7953d8 as libc::c_ulong,
            0x49e14f17 as libc::c_ulong,
            0x50fa7e56 as libc::c_ulong,
            0x7bd72d95 as libc::c_ulong,
            0x62cc1cd4 as libc::c_ulong,
            0x2d8d8a13 as libc::c_ulong,
            0x3496bb52 as libc::c_ulong,
            0x1fbbe891 as libc::c_ulong,
            0x6a0d9d0 as libc::c_ulong,
            0x5e7ef3ec as libc::c_ulong,
            0x4765c2ad as libc::c_ulong,
            0x6c48916e as libc::c_ulong,
            0x7553a02f as libc::c_ulong,
            0x3a1236e8 as libc::c_ulong,
            0x230907a9 as libc::c_ulong,
            0x824546a as libc::c_ulong,
            0x113f652b as libc::c_ulong,
            0x96a779e4 as libc::c_ulong,
            0x8fbc48a5 as libc::c_ulong,
            0xa4911b66 as libc::c_ulong,
            0xbd8a2a27 as libc::c_ulong,
            0xf2cbbce0 as libc::c_ulong,
            0xebd08da1 as libc::c_ulong,
            0xc0fdde62 as libc::c_ulong,
            0xd9e6ef23 as libc::c_ulong,
            0x14bce1bd as libc::c_ulong,
            0xda7d0fc as libc::c_ulong,
            0x268a833f as libc::c_ulong,
            0x3f91b27e as libc::c_ulong,
            0x70d024b9 as libc::c_ulong,
            0x69cb15f8 as libc::c_ulong,
            0x42e6463b as libc::c_ulong,
            0x5bfd777a as libc::c_ulong,
            0xdc656bb5 as libc::c_ulong,
            0xc57e5af4 as libc::c_ulong,
            0xee530937 as libc::c_ulong,
            0xf7483876 as libc::c_ulong,
            0xb809aeb1 as libc::c_ulong,
            0xa1129ff0 as libc::c_ulong,
            0x8a3fcc33 as libc::c_ulong,
            0x9324fd72 as libc::c_ulong,
        ],
        [
            0 as libc::c_ulong,
            0x1c26a37 as libc::c_ulong,
            0x384d46e as libc::c_ulong,
            0x246be59 as libc::c_ulong,
            0x709a8dc as libc::c_ulong,
            0x6cbc2eb as libc::c_ulong,
            0x48d7cb2 as libc::c_ulong,
            0x54f1685 as libc::c_ulong,
            0xe1351b8 as libc::c_ulong,
            0xfd13b8f as libc::c_ulong,
            0xd9785d6 as libc::c_ulong,
            0xc55efe1 as libc::c_ulong,
            0x91af964 as libc::c_ulong,
            0x8d89353 as libc::c_ulong,
            0xa9e2d0a as libc::c_ulong,
            0xb5c473d as libc::c_ulong,
            0x1c26a370 as libc::c_ulong,
            0x1de4c947 as libc::c_ulong,
            0x1fa2771e as libc::c_ulong,
            0x1e601d29 as libc::c_ulong,
            0x1b2f0bac as libc::c_ulong,
            0x1aed619b as libc::c_ulong,
            0x18abdfc2 as libc::c_ulong,
            0x1969b5f5 as libc::c_ulong,
            0x1235f2c8 as libc::c_ulong,
            0x13f798ff as libc::c_ulong,
            0x11b126a6 as libc::c_ulong,
            0x10734c91 as libc::c_ulong,
            0x153c5a14 as libc::c_ulong,
            0x14fe3023 as libc::c_ulong,
            0x16b88e7a as libc::c_ulong,
            0x177ae44d as libc::c_ulong,
            0x384d46e0 as libc::c_ulong,
            0x398f2cd7 as libc::c_ulong,
            0x3bc9928e as libc::c_ulong,
            0x3a0bf8b9 as libc::c_ulong,
            0x3f44ee3c as libc::c_ulong,
            0x3e86840b as libc::c_ulong,
            0x3cc03a52 as libc::c_ulong,
            0x3d025065 as libc::c_ulong,
            0x365e1758 as libc::c_ulong,
            0x379c7d6f as libc::c_ulong,
            0x35dac336 as libc::c_ulong,
            0x3418a901 as libc::c_ulong,
            0x3157bf84 as libc::c_ulong,
            0x3095d5b3 as libc::c_ulong,
            0x32d36bea as libc::c_ulong,
            0x331101dd as libc::c_ulong,
            0x246be590 as libc::c_ulong,
            0x25a98fa7 as libc::c_ulong,
            0x27ef31fe as libc::c_ulong,
            0x262d5bc9 as libc::c_ulong,
            0x23624d4c as libc::c_ulong,
            0x22a0277b as libc::c_ulong,
            0x20e69922 as libc::c_ulong,
            0x2124f315 as libc::c_ulong,
            0x2a78b428 as libc::c_ulong,
            0x2bbade1f as libc::c_ulong,
            0x29fc6046 as libc::c_ulong,
            0x283e0a71 as libc::c_ulong,
            0x2d711cf4 as libc::c_ulong,
            0x2cb376c3 as libc::c_ulong,
            0x2ef5c89a as libc::c_ulong,
            0x2f37a2ad as libc::c_ulong,
            0x709a8dc0 as libc::c_ulong,
            0x7158e7f7 as libc::c_ulong,
            0x731e59ae as libc::c_ulong,
            0x72dc3399 as libc::c_ulong,
            0x7793251c as libc::c_ulong,
            0x76514f2b as libc::c_ulong,
            0x7417f172 as libc::c_ulong,
            0x75d59b45 as libc::c_ulong,
            0x7e89dc78 as libc::c_ulong,
            0x7f4bb64f as libc::c_ulong,
            0x7d0d0816 as libc::c_ulong,
            0x7ccf6221 as libc::c_ulong,
            0x798074a4 as libc::c_ulong,
            0x78421e93 as libc::c_ulong,
            0x7a04a0ca as libc::c_ulong,
            0x7bc6cafd as libc::c_ulong,
            0x6cbc2eb0 as libc::c_ulong,
            0x6d7e4487 as libc::c_ulong,
            0x6f38fade as libc::c_ulong,
            0x6efa90e9 as libc::c_ulong,
            0x6bb5866c as libc::c_ulong,
            0x6a77ec5b as libc::c_ulong,
            0x68315202 as libc::c_ulong,
            0x69f33835 as libc::c_ulong,
            0x62af7f08 as libc::c_ulong,
            0x636d153f as libc::c_ulong,
            0x612bab66 as libc::c_ulong,
            0x60e9c151 as libc::c_ulong,
            0x65a6d7d4 as libc::c_ulong,
            0x6464bde3 as libc::c_ulong,
            0x662203ba as libc::c_ulong,
            0x67e0698d as libc::c_ulong,
            0x48d7cb20 as libc::c_ulong,
            0x4915a117 as libc::c_ulong,
            0x4b531f4e as libc::c_ulong,
            0x4a917579 as libc::c_ulong,
            0x4fde63fc as libc::c_ulong,
            0x4e1c09cb as libc::c_ulong,
            0x4c5ab792 as libc::c_ulong,
            0x4d98dda5 as libc::c_ulong,
            0x46c49a98 as libc::c_ulong,
            0x4706f0af as libc::c_ulong,
            0x45404ef6 as libc::c_ulong,
            0x448224c1 as libc::c_ulong,
            0x41cd3244 as libc::c_ulong,
            0x400f5873 as libc::c_ulong,
            0x4249e62a as libc::c_ulong,
            0x438b8c1d as libc::c_ulong,
            0x54f16850 as libc::c_ulong,
            0x55330267 as libc::c_ulong,
            0x5775bc3e as libc::c_ulong,
            0x56b7d609 as libc::c_ulong,
            0x53f8c08c as libc::c_ulong,
            0x523aaabb as libc::c_ulong,
            0x507c14e2 as libc::c_ulong,
            0x51be7ed5 as libc::c_ulong,
            0x5ae239e8 as libc::c_ulong,
            0x5b2053df as libc::c_ulong,
            0x5966ed86 as libc::c_ulong,
            0x58a487b1 as libc::c_ulong,
            0x5deb9134 as libc::c_ulong,
            0x5c29fb03 as libc::c_ulong,
            0x5e6f455a as libc::c_ulong,
            0x5fad2f6d as libc::c_ulong,
            0xe1351b80 as libc::c_ulong,
            0xe0f771b7 as libc::c_ulong,
            0xe2b1cfee as libc::c_ulong,
            0xe373a5d9 as libc::c_ulong,
            0xe63cb35c as libc::c_ulong,
            0xe7fed96b as libc::c_ulong,
            0xe5b86732 as libc::c_ulong,
            0xe47a0d05 as libc::c_ulong,
            0xef264a38 as libc::c_ulong,
            0xeee4200f as libc::c_ulong,
            0xeca29e56 as libc::c_ulong,
            0xed60f461 as libc::c_ulong,
            0xe82fe2e4 as libc::c_ulong,
            0xe9ed88d3 as libc::c_ulong,
            0xebab368a as libc::c_ulong,
            0xea695cbd as libc::c_ulong,
            0xfd13b8f0 as libc::c_ulong,
            0xfcd1d2c7 as libc::c_ulong,
            0xfe976c9e as libc::c_ulong,
            0xff5506a9 as libc::c_ulong,
            0xfa1a102c as libc::c_ulong,
            0xfbd87a1b as libc::c_ulong,
            0xf99ec442 as libc::c_ulong,
            0xf85cae75 as libc::c_ulong,
            0xf300e948 as libc::c_ulong,
            0xf2c2837f as libc::c_ulong,
            0xf0843d26 as libc::c_ulong,
            0xf1465711 as libc::c_ulong,
            0xf4094194 as libc::c_ulong,
            0xf5cb2ba3 as libc::c_ulong,
            0xf78d95fa as libc::c_ulong,
            0xf64fffcd as libc::c_ulong,
            0xd9785d60 as libc::c_ulong,
            0xd8ba3757 as libc::c_ulong,
            0xdafc890e as libc::c_ulong,
            0xdb3ee339 as libc::c_ulong,
            0xde71f5bc as libc::c_ulong,
            0xdfb39f8b as libc::c_ulong,
            0xddf521d2 as libc::c_ulong,
            0xdc374be5 as libc::c_ulong,
            0xd76b0cd8 as libc::c_ulong,
            0xd6a966ef as libc::c_ulong,
            0xd4efd8b6 as libc::c_ulong,
            0xd52db281 as libc::c_ulong,
            0xd062a404 as libc::c_ulong,
            0xd1a0ce33 as libc::c_ulong,
            0xd3e6706a as libc::c_ulong,
            0xd2241a5d as libc::c_ulong,
            0xc55efe10 as libc::c_ulong,
            0xc49c9427 as libc::c_ulong,
            0xc6da2a7e as libc::c_ulong,
            0xc7184049 as libc::c_ulong,
            0xc25756cc as libc::c_ulong,
            0xc3953cfb as libc::c_ulong,
            0xc1d382a2 as libc::c_ulong,
            0xc011e895 as libc::c_ulong,
            0xcb4dafa8 as libc::c_ulong,
            0xca8fc59f as libc::c_ulong,
            0xc8c97bc6 as libc::c_ulong,
            0xc90b11f1 as libc::c_ulong,
            0xcc440774 as libc::c_ulong,
            0xcd866d43 as libc::c_ulong,
            0xcfc0d31a as libc::c_ulong,
            0xce02b92d as libc::c_ulong,
            0x91af9640 as libc::c_ulong,
            0x906dfc77 as libc::c_ulong,
            0x922b422e as libc::c_ulong,
            0x93e92819 as libc::c_ulong,
            0x96a63e9c as libc::c_ulong,
            0x976454ab as libc::c_ulong,
            0x9522eaf2 as libc::c_ulong,
            0x94e080c5 as libc::c_ulong,
            0x9fbcc7f8 as libc::c_ulong,
            0x9e7eadcf as libc::c_ulong,
            0x9c381396 as libc::c_ulong,
            0x9dfa79a1 as libc::c_ulong,
            0x98b56f24 as libc::c_ulong,
            0x99770513 as libc::c_ulong,
            0x9b31bb4a as libc::c_ulong,
            0x9af3d17d as libc::c_ulong,
            0x8d893530 as libc::c_ulong,
            0x8c4b5f07 as libc::c_ulong,
            0x8e0de15e as libc::c_ulong,
            0x8fcf8b69 as libc::c_ulong,
            0x8a809dec as libc::c_ulong,
            0x8b42f7db as libc::c_ulong,
            0x89044982 as libc::c_ulong,
            0x88c623b5 as libc::c_ulong,
            0x839a6488 as libc::c_ulong,
            0x82580ebf as libc::c_ulong,
            0x801eb0e6 as libc::c_ulong,
            0x81dcdad1 as libc::c_ulong,
            0x8493cc54 as libc::c_ulong,
            0x8551a663 as libc::c_ulong,
            0x8717183a as libc::c_ulong,
            0x86d5720d as libc::c_ulong,
            0xa9e2d0a0 as libc::c_ulong,
            0xa820ba97 as libc::c_ulong,
            0xaa6604ce as libc::c_ulong,
            0xaba46ef9 as libc::c_ulong,
            0xaeeb787c as libc::c_ulong,
            0xaf29124b as libc::c_ulong,
            0xad6fac12 as libc::c_ulong,
            0xacadc625 as libc::c_ulong,
            0xa7f18118 as libc::c_ulong,
            0xa633eb2f as libc::c_ulong,
            0xa4755576 as libc::c_ulong,
            0xa5b73f41 as libc::c_ulong,
            0xa0f829c4 as libc::c_ulong,
            0xa13a43f3 as libc::c_ulong,
            0xa37cfdaa as libc::c_ulong,
            0xa2be979d as libc::c_ulong,
            0xb5c473d0 as libc::c_ulong,
            0xb40619e7 as libc::c_ulong,
            0xb640a7be as libc::c_ulong,
            0xb782cd89 as libc::c_ulong,
            0xb2cddb0c as libc::c_ulong,
            0xb30fb13b as libc::c_ulong,
            0xb1490f62 as libc::c_ulong,
            0xb08b6555 as libc::c_ulong,
            0xbbd72268 as libc::c_ulong,
            0xba15485f as libc::c_ulong,
            0xb853f606 as libc::c_ulong,
            0xb9919c31 as libc::c_ulong,
            0xbcde8ab4 as libc::c_ulong,
            0xbd1ce083 as libc::c_ulong,
            0xbf5a5eda as libc::c_ulong,
            0xbe9834ed as libc::c_ulong,
        ],
        [
            0 as libc::c_ulong,
            0xb8bc6765 as libc::c_ulong,
            0xaa09c88b as libc::c_ulong,
            0x12b5afee as libc::c_ulong,
            0x8f629757 as libc::c_ulong,
            0x37def032 as libc::c_ulong,
            0x256b5fdc as libc::c_ulong,
            0x9dd738b9 as libc::c_ulong,
            0xc5b428ef as libc::c_ulong,
            0x7d084f8a as libc::c_ulong,
            0x6fbde064 as libc::c_ulong,
            0xd7018701 as libc::c_ulong,
            0x4ad6bfb8 as libc::c_ulong,
            0xf26ad8dd as libc::c_ulong,
            0xe0df7733 as libc::c_ulong,
            0x58631056 as libc::c_ulong,
            0x5019579f as libc::c_ulong,
            0xe8a530fa as libc::c_ulong,
            0xfa109f14 as libc::c_ulong,
            0x42acf871 as libc::c_ulong,
            0xdf7bc0c8 as libc::c_ulong,
            0x67c7a7ad as libc::c_ulong,
            0x75720843 as libc::c_ulong,
            0xcdce6f26 as libc::c_ulong,
            0x95ad7f70 as libc::c_ulong,
            0x2d111815 as libc::c_ulong,
            0x3fa4b7fb as libc::c_ulong,
            0x8718d09e as libc::c_ulong,
            0x1acfe827 as libc::c_ulong,
            0xa2738f42 as libc::c_ulong,
            0xb0c620ac as libc::c_ulong,
            0x87a47c9 as libc::c_ulong,
            0xa032af3e as libc::c_ulong,
            0x188ec85b as libc::c_ulong,
            0xa3b67b5 as libc::c_ulong,
            0xb28700d0 as libc::c_ulong,
            0x2f503869 as libc::c_ulong,
            0x97ec5f0c as libc::c_ulong,
            0x8559f0e2 as libc::c_ulong,
            0x3de59787 as libc::c_ulong,
            0x658687d1 as libc::c_ulong,
            0xdd3ae0b4 as libc::c_ulong,
            0xcf8f4f5a as libc::c_ulong,
            0x7733283f as libc::c_ulong,
            0xeae41086 as libc::c_ulong,
            0x525877e3 as libc::c_ulong,
            0x40edd80d as libc::c_ulong,
            0xf851bf68 as libc::c_ulong,
            0xf02bf8a1 as libc::c_ulong,
            0x48979fc4 as libc::c_ulong,
            0x5a22302a as libc::c_ulong,
            0xe29e574f as libc::c_ulong,
            0x7f496ff6 as libc::c_ulong,
            0xc7f50893 as libc::c_ulong,
            0xd540a77d as libc::c_ulong,
            0x6dfcc018 as libc::c_ulong,
            0x359fd04e as libc::c_ulong,
            0x8d23b72b as libc::c_ulong,
            0x9f9618c5 as libc::c_ulong,
            0x272a7fa0 as libc::c_ulong,
            0xbafd4719 as libc::c_ulong,
            0x241207c as libc::c_ulong,
            0x10f48f92 as libc::c_ulong,
            0xa848e8f7 as libc::c_ulong,
            0x9b14583d as libc::c_ulong,
            0x23a83f58 as libc::c_ulong,
            0x311d90b6 as libc::c_ulong,
            0x89a1f7d3 as libc::c_ulong,
            0x1476cf6a as libc::c_ulong,
            0xaccaa80f as libc::c_ulong,
            0xbe7f07e1 as libc::c_ulong,
            0x6c36084 as libc::c_ulong,
            0x5ea070d2 as libc::c_ulong,
            0xe61c17b7 as libc::c_ulong,
            0xf4a9b859 as libc::c_ulong,
            0x4c15df3c as libc::c_ulong,
            0xd1c2e785 as libc::c_ulong,
            0x697e80e0 as libc::c_ulong,
            0x7bcb2f0e as libc::c_ulong,
            0xc377486b as libc::c_ulong,
            0xcb0d0fa2 as libc::c_ulong,
            0x73b168c7 as libc::c_ulong,
            0x6104c729 as libc::c_ulong,
            0xd9b8a04c as libc::c_ulong,
            0x446f98f5 as libc::c_ulong,
            0xfcd3ff90 as libc::c_ulong,
            0xee66507e as libc::c_ulong,
            0x56da371b as libc::c_ulong,
            0xeb9274d as libc::c_ulong,
            0xb6054028 as libc::c_ulong,
            0xa4b0efc6 as libc::c_ulong,
            0x1c0c88a3 as libc::c_ulong,
            0x81dbb01a as libc::c_ulong,
            0x3967d77f as libc::c_ulong,
            0x2bd27891 as libc::c_ulong,
            0x936e1ff4 as libc::c_ulong,
            0x3b26f703 as libc::c_ulong,
            0x839a9066 as libc::c_ulong,
            0x912f3f88 as libc::c_ulong,
            0x299358ed as libc::c_ulong,
            0xb4446054 as libc::c_ulong,
            0xcf80731 as libc::c_ulong,
            0x1e4da8df as libc::c_ulong,
            0xa6f1cfba as libc::c_ulong,
            0xfe92dfec as libc::c_ulong,
            0x462eb889 as libc::c_ulong,
            0x549b1767 as libc::c_ulong,
            0xec277002 as libc::c_ulong,
            0x71f048bb as libc::c_ulong,
            0xc94c2fde as libc::c_ulong,
            0xdbf98030 as libc::c_ulong,
            0x6345e755 as libc::c_ulong,
            0x6b3fa09c as libc::c_ulong,
            0xd383c7f9 as libc::c_ulong,
            0xc1366817 as libc::c_ulong,
            0x798a0f72 as libc::c_ulong,
            0xe45d37cb as libc::c_ulong,
            0x5ce150ae as libc::c_ulong,
            0x4e54ff40 as libc::c_ulong,
            0xf6e89825 as libc::c_ulong,
            0xae8b8873 as libc::c_ulong,
            0x1637ef16 as libc::c_ulong,
            0x48240f8 as libc::c_ulong,
            0xbc3e279d as libc::c_ulong,
            0x21e91f24 as libc::c_ulong,
            0x99557841 as libc::c_ulong,
            0x8be0d7af as libc::c_ulong,
            0x335cb0ca as libc::c_ulong,
            0xed59b63b as libc::c_ulong,
            0x55e5d15e as libc::c_ulong,
            0x47507eb0 as libc::c_ulong,
            0xffec19d5 as libc::c_ulong,
            0x623b216c as libc::c_ulong,
            0xda874609 as libc::c_ulong,
            0xc832e9e7 as libc::c_ulong,
            0x708e8e82 as libc::c_ulong,
            0x28ed9ed4 as libc::c_ulong,
            0x9051f9b1 as libc::c_ulong,
            0x82e4565f as libc::c_ulong,
            0x3a58313a as libc::c_ulong,
            0xa78f0983 as libc::c_ulong,
            0x1f336ee6 as libc::c_ulong,
            0xd86c108 as libc::c_ulong,
            0xb53aa66d as libc::c_ulong,
            0xbd40e1a4 as libc::c_ulong,
            0x5fc86c1 as libc::c_ulong,
            0x1749292f as libc::c_ulong,
            0xaff54e4a as libc::c_ulong,
            0x322276f3 as libc::c_ulong,
            0x8a9e1196 as libc::c_ulong,
            0x982bbe78 as libc::c_ulong,
            0x2097d91d as libc::c_ulong,
            0x78f4c94b as libc::c_ulong,
            0xc048ae2e as libc::c_ulong,
            0xd2fd01c0 as libc::c_ulong,
            0x6a4166a5 as libc::c_ulong,
            0xf7965e1c as libc::c_ulong,
            0x4f2a3979 as libc::c_ulong,
            0x5d9f9697 as libc::c_ulong,
            0xe523f1f2 as libc::c_ulong,
            0x4d6b1905 as libc::c_ulong,
            0xf5d77e60 as libc::c_ulong,
            0xe762d18e as libc::c_ulong,
            0x5fdeb6eb as libc::c_ulong,
            0xc2098e52 as libc::c_ulong,
            0x7ab5e937 as libc::c_ulong,
            0x680046d9 as libc::c_ulong,
            0xd0bc21bc as libc::c_ulong,
            0x88df31ea as libc::c_ulong,
            0x3063568f as libc::c_ulong,
            0x22d6f961 as libc::c_ulong,
            0x9a6a9e04 as libc::c_ulong,
            0x7bda6bd as libc::c_ulong,
            0xbf01c1d8 as libc::c_ulong,
            0xadb46e36 as libc::c_ulong,
            0x15080953 as libc::c_ulong,
            0x1d724e9a as libc::c_ulong,
            0xa5ce29ff as libc::c_ulong,
            0xb77b8611 as libc::c_ulong,
            0xfc7e174 as libc::c_ulong,
            0x9210d9cd as libc::c_ulong,
            0x2aacbea8 as libc::c_ulong,
            0x38191146 as libc::c_ulong,
            0x80a57623 as libc::c_ulong,
            0xd8c66675 as libc::c_ulong,
            0x607a0110 as libc::c_ulong,
            0x72cfaefe as libc::c_ulong,
            0xca73c99b as libc::c_ulong,
            0x57a4f122 as libc::c_ulong,
            0xef189647 as libc::c_ulong,
            0xfdad39a9 as libc::c_ulong,
            0x45115ecc as libc::c_ulong,
            0x764dee06 as libc::c_ulong,
            0xcef18963 as libc::c_ulong,
            0xdc44268d as libc::c_ulong,
            0x64f841e8 as libc::c_ulong,
            0xf92f7951 as libc::c_ulong,
            0x41931e34 as libc::c_ulong,
            0x5326b1da as libc::c_ulong,
            0xeb9ad6bf as libc::c_ulong,
            0xb3f9c6e9 as libc::c_ulong,
            0xb45a18c as libc::c_ulong,
            0x19f00e62 as libc::c_ulong,
            0xa14c6907 as libc::c_ulong,
            0x3c9b51be as libc::c_ulong,
            0x842736db as libc::c_ulong,
            0x96929935 as libc::c_ulong,
            0x2e2efe50 as libc::c_ulong,
            0x2654b999 as libc::c_ulong,
            0x9ee8defc as libc::c_ulong,
            0x8c5d7112 as libc::c_ulong,
            0x34e11677 as libc::c_ulong,
            0xa9362ece as libc::c_ulong,
            0x118a49ab as libc::c_ulong,
            0x33fe645 as libc::c_ulong,
            0xbb838120 as libc::c_ulong,
            0xe3e09176 as libc::c_ulong,
            0x5b5cf613 as libc::c_ulong,
            0x49e959fd as libc::c_ulong,
            0xf1553e98 as libc::c_ulong,
            0x6c820621 as libc::c_ulong,
            0xd43e6144 as libc::c_ulong,
            0xc68bceaa as libc::c_ulong,
            0x7e37a9cf as libc::c_ulong,
            0xd67f4138 as libc::c_ulong,
            0x6ec3265d as libc::c_ulong,
            0x7c7689b3 as libc::c_ulong,
            0xc4caeed6 as libc::c_ulong,
            0x591dd66f as libc::c_ulong,
            0xe1a1b10a as libc::c_ulong,
            0xf3141ee4 as libc::c_ulong,
            0x4ba87981 as libc::c_ulong,
            0x13cb69d7 as libc::c_ulong,
            0xab770eb2 as libc::c_ulong,
            0xb9c2a15c as libc::c_ulong,
            0x17ec639 as libc::c_ulong,
            0x9ca9fe80 as libc::c_ulong,
            0x241599e5 as libc::c_ulong,
            0x36a0360b as libc::c_ulong,
            0x8e1c516e as libc::c_ulong,
            0x866616a7 as libc::c_ulong,
            0x3eda71c2 as libc::c_ulong,
            0x2c6fde2c as libc::c_ulong,
            0x94d3b949 as libc::c_ulong,
            0x90481f0 as libc::c_ulong,
            0xb1b8e695 as libc::c_ulong,
            0xa30d497b as libc::c_ulong,
            0x1bb12e1e as libc::c_ulong,
            0x43d23e48 as libc::c_ulong,
            0xfb6e592d as libc::c_ulong,
            0xe9dbf6c3 as libc::c_ulong,
            0x516791a6 as libc::c_ulong,
            0xccb0a91f as libc::c_ulong,
            0x740cce7a as libc::c_ulong,
            0x66b96194 as libc::c_ulong,
            0xde0506f1 as libc::c_ulong,
        ],
        [
            0 as libc::c_ulong,
            0x96300777 as libc::c_ulong,
            0x2c610eee as libc::c_ulong,
            0xba510999 as libc::c_ulong,
            0x19c46d07 as libc::c_ulong,
            0x8ff46a70 as libc::c_ulong,
            0x35a563e9 as libc::c_ulong,
            0xa395649e as libc::c_ulong,
            0x3288db0e as libc::c_ulong,
            0xa4b8dc79 as libc::c_ulong,
            0x1ee9d5e0 as libc::c_ulong,
            0x88d9d297 as libc::c_ulong,
            0x2b4cb609 as libc::c_ulong,
            0xbd7cb17e as libc::c_ulong,
            0x72db8e7 as libc::c_ulong,
            0x911dbf90 as libc::c_ulong,
            0x6410b71d as libc::c_ulong,
            0xf220b06a as libc::c_ulong,
            0x4871b9f3 as libc::c_ulong,
            0xde41be84 as libc::c_ulong,
            0x7dd4da1a as libc::c_ulong,
            0xebe4dd6d as libc::c_ulong,
            0x51b5d4f4 as libc::c_ulong,
            0xc785d383 as libc::c_ulong,
            0x56986c13 as libc::c_ulong,
            0xc0a86b64 as libc::c_ulong,
            0x7af962fd as libc::c_ulong,
            0xecc9658a as libc::c_ulong,
            0x4f5c0114 as libc::c_ulong,
            0xd96c0663 as libc::c_ulong,
            0x633d0ffa as libc::c_ulong,
            0xf50d088d as libc::c_ulong,
            0xc8206e3b as libc::c_ulong,
            0x5e10694c as libc::c_ulong,
            0xe44160d5 as libc::c_ulong,
            0x727167a2 as libc::c_ulong,
            0xd1e4033c as libc::c_ulong,
            0x47d4044b as libc::c_ulong,
            0xfd850dd2 as libc::c_ulong,
            0x6bb50aa5 as libc::c_ulong,
            0xfaa8b535 as libc::c_ulong,
            0x6c98b242 as libc::c_ulong,
            0xd6c9bbdb as libc::c_ulong,
            0x40f9bcac as libc::c_ulong,
            0xe36cd832 as libc::c_ulong,
            0x755cdf45 as libc::c_ulong,
            0xcf0dd6dc as libc::c_ulong,
            0x593dd1ab as libc::c_ulong,
            0xac30d926 as libc::c_ulong,
            0x3a00de51 as libc::c_ulong,
            0x8051d7c8 as libc::c_ulong,
            0x1661d0bf as libc::c_ulong,
            0xb5f4b421 as libc::c_ulong,
            0x23c4b356 as libc::c_ulong,
            0x9995bacf as libc::c_ulong,
            0xfa5bdb8 as libc::c_ulong,
            0x9eb80228 as libc::c_ulong,
            0x888055f as libc::c_ulong,
            0xb2d90cc6 as libc::c_ulong,
            0x24e90bb1 as libc::c_ulong,
            0x877c6f2f as libc::c_ulong,
            0x114c6858 as libc::c_ulong,
            0xab1d61c1 as libc::c_ulong,
            0x3d2d66b6 as libc::c_ulong,
            0x9041dc76 as libc::c_ulong,
            0x671db01 as libc::c_ulong,
            0xbc20d298 as libc::c_ulong,
            0x2a10d5ef as libc::c_ulong,
            0x8985b171 as libc::c_ulong,
            0x1fb5b606 as libc::c_ulong,
            0xa5e4bf9f as libc::c_ulong,
            0x33d4b8e8 as libc::c_ulong,
            0xa2c90778 as libc::c_ulong,
            0x34f9000f as libc::c_ulong,
            0x8ea80996 as libc::c_ulong,
            0x18980ee1 as libc::c_ulong,
            0xbb0d6a7f as libc::c_ulong,
            0x2d3d6d08 as libc::c_ulong,
            0x976c6491 as libc::c_ulong,
            0x15c63e6 as libc::c_ulong,
            0xf4516b6b as libc::c_ulong,
            0x62616c1c as libc::c_ulong,
            0xd8306585 as libc::c_ulong,
            0x4e0062f2 as libc::c_ulong,
            0xed95066c as libc::c_ulong,
            0x7ba5011b as libc::c_ulong,
            0xc1f40882 as libc::c_ulong,
            0x57c40ff5 as libc::c_ulong,
            0xc6d9b065 as libc::c_ulong,
            0x50e9b712 as libc::c_ulong,
            0xeab8be8b as libc::c_ulong,
            0x7c88b9fc as libc::c_ulong,
            0xdf1ddd62 as libc::c_ulong,
            0x492dda15 as libc::c_ulong,
            0xf37cd38c as libc::c_ulong,
            0x654cd4fb as libc::c_ulong,
            0x5861b24d as libc::c_ulong,
            0xce51b53a as libc::c_ulong,
            0x7400bca3 as libc::c_ulong,
            0xe230bbd4 as libc::c_ulong,
            0x41a5df4a as libc::c_ulong,
            0xd795d83d as libc::c_ulong,
            0x6dc4d1a4 as libc::c_ulong,
            0xfbf4d6d3 as libc::c_ulong,
            0x6ae96943 as libc::c_ulong,
            0xfcd96e34 as libc::c_ulong,
            0x468867ad as libc::c_ulong,
            0xd0b860da as libc::c_ulong,
            0x732d0444 as libc::c_ulong,
            0xe51d0333 as libc::c_ulong,
            0x5f4c0aaa as libc::c_ulong,
            0xc97c0ddd as libc::c_ulong,
            0x3c710550 as libc::c_ulong,
            0xaa410227 as libc::c_ulong,
            0x10100bbe as libc::c_ulong,
            0x86200cc9 as libc::c_ulong,
            0x25b56857 as libc::c_ulong,
            0xb3856f20 as libc::c_ulong,
            0x9d466b9 as libc::c_ulong,
            0x9fe461ce as libc::c_ulong,
            0xef9de5e as libc::c_ulong,
            0x98c9d929 as libc::c_ulong,
            0x2298d0b0 as libc::c_ulong,
            0xb4a8d7c7 as libc::c_ulong,
            0x173db359 as libc::c_ulong,
            0x810db42e as libc::c_ulong,
            0x3b5cbdb7 as libc::c_ulong,
            0xad6cbac0 as libc::c_ulong,
            0x2083b8ed as libc::c_ulong,
            0xb6b3bf9a as libc::c_ulong,
            0xce2b603 as libc::c_ulong,
            0x9ad2b174 as libc::c_ulong,
            0x3947d5ea as libc::c_ulong,
            0xaf77d29d as libc::c_ulong,
            0x1526db04 as libc::c_ulong,
            0x8316dc73 as libc::c_ulong,
            0x120b63e3 as libc::c_ulong,
            0x843b6494 as libc::c_ulong,
            0x3e6a6d0d as libc::c_ulong,
            0xa85a6a7a as libc::c_ulong,
            0xbcf0ee4 as libc::c_ulong,
            0x9dff0993 as libc::c_ulong,
            0x27ae000a as libc::c_ulong,
            0xb19e077d as libc::c_ulong,
            0x44930ff0 as libc::c_ulong,
            0xd2a30887 as libc::c_ulong,
            0x68f2011e as libc::c_ulong,
            0xfec20669 as libc::c_ulong,
            0x5d5762f7 as libc::c_ulong,
            0xcb676580 as libc::c_ulong,
            0x71366c19 as libc::c_ulong,
            0xe7066b6e as libc::c_ulong,
            0x761bd4fe as libc::c_ulong,
            0xe02bd389 as libc::c_ulong,
            0x5a7ada10 as libc::c_ulong,
            0xcc4add67 as libc::c_ulong,
            0x6fdfb9f9 as libc::c_ulong,
            0xf9efbe8e as libc::c_ulong,
            0x43beb717 as libc::c_ulong,
            0xd58eb060 as libc::c_ulong,
            0xe8a3d6d6 as libc::c_ulong,
            0x7e93d1a1 as libc::c_ulong,
            0xc4c2d838 as libc::c_ulong,
            0x52f2df4f as libc::c_ulong,
            0xf167bbd1 as libc::c_ulong,
            0x6757bca6 as libc::c_ulong,
            0xdd06b53f as libc::c_ulong,
            0x4b36b248 as libc::c_ulong,
            0xda2b0dd8 as libc::c_ulong,
            0x4c1b0aaf as libc::c_ulong,
            0xf64a0336 as libc::c_ulong,
            0x607a0441 as libc::c_ulong,
            0xc3ef60df as libc::c_ulong,
            0x55df67a8 as libc::c_ulong,
            0xef8e6e31 as libc::c_ulong,
            0x79be6946 as libc::c_ulong,
            0x8cb361cb as libc::c_ulong,
            0x1a8366bc as libc::c_ulong,
            0xa0d26f25 as libc::c_ulong,
            0x36e26852 as libc::c_ulong,
            0x95770ccc as libc::c_ulong,
            0x3470bbb as libc::c_ulong,
            0xb9160222 as libc::c_ulong,
            0x2f260555 as libc::c_ulong,
            0xbe3bbac5 as libc::c_ulong,
            0x280bbdb2 as libc::c_ulong,
            0x925ab42b as libc::c_ulong,
            0x46ab35c as libc::c_ulong,
            0xa7ffd7c2 as libc::c_ulong,
            0x31cfd0b5 as libc::c_ulong,
            0x8b9ed92c as libc::c_ulong,
            0x1daede5b as libc::c_ulong,
            0xb0c2649b as libc::c_ulong,
            0x26f263ec as libc::c_ulong,
            0x9ca36a75 as libc::c_ulong,
            0xa936d02 as libc::c_ulong,
            0xa906099c as libc::c_ulong,
            0x3f360eeb as libc::c_ulong,
            0x85670772 as libc::c_ulong,
            0x13570005 as libc::c_ulong,
            0x824abf95 as libc::c_ulong,
            0x147ab8e2 as libc::c_ulong,
            0xae2bb17b as libc::c_ulong,
            0x381bb60c as libc::c_ulong,
            0x9b8ed292 as libc::c_ulong,
            0xdbed5e5 as libc::c_ulong,
            0xb7efdc7c as libc::c_ulong,
            0x21dfdb0b as libc::c_ulong,
            0xd4d2d386 as libc::c_ulong,
            0x42e2d4f1 as libc::c_ulong,
            0xf8b3dd68 as libc::c_ulong,
            0x6e83da1f as libc::c_ulong,
            0xcd16be81 as libc::c_ulong,
            0x5b26b9f6 as libc::c_ulong,
            0xe177b06f as libc::c_ulong,
            0x7747b718 as libc::c_ulong,
            0xe65a0888 as libc::c_ulong,
            0x706a0fff as libc::c_ulong,
            0xca3b0666 as libc::c_ulong,
            0x5c0b0111 as libc::c_ulong,
            0xff9e658f as libc::c_ulong,
            0x69ae62f8 as libc::c_ulong,
            0xd3ff6b61 as libc::c_ulong,
            0x45cf6c16 as libc::c_ulong,
            0x78e20aa0 as libc::c_ulong,
            0xeed20dd7 as libc::c_ulong,
            0x5483044e as libc::c_ulong,
            0xc2b30339 as libc::c_ulong,
            0x612667a7 as libc::c_ulong,
            0xf71660d0 as libc::c_ulong,
            0x4d476949 as libc::c_ulong,
            0xdb776e3e as libc::c_ulong,
            0x4a6ad1ae as libc::c_ulong,
            0xdc5ad6d9 as libc::c_ulong,
            0x660bdf40 as libc::c_ulong,
            0xf03bd837 as libc::c_ulong,
            0x53aebca9 as libc::c_ulong,
            0xc59ebbde as libc::c_ulong,
            0x7fcfb247 as libc::c_ulong,
            0xe9ffb530 as libc::c_ulong,
            0x1cf2bdbd as libc::c_ulong,
            0x8ac2baca as libc::c_ulong,
            0x3093b353 as libc::c_ulong,
            0xa6a3b424 as libc::c_ulong,
            0x536d0ba as libc::c_ulong,
            0x9306d7cd as libc::c_ulong,
            0x2957de54 as libc::c_ulong,
            0xbf67d923 as libc::c_ulong,
            0x2e7a66b3 as libc::c_ulong,
            0xb84a61c4 as libc::c_ulong,
            0x21b685d as libc::c_ulong,
            0x942b6f2a as libc::c_ulong,
            0x37be0bb4 as libc::c_ulong,
            0xa18e0cc3 as libc::c_ulong,
            0x1bdf055a as libc::c_ulong,
            0x8def022d as libc::c_ulong,
        ],
        [
            0 as libc::c_ulong,
            0x41311b19 as libc::c_ulong,
            0x82623632 as libc::c_ulong,
            0xc3532d2b as libc::c_ulong,
            0x4c56c64 as libc::c_ulong,
            0x45f4777d as libc::c_ulong,
            0x86a75a56 as libc::c_ulong,
            0xc796414f as libc::c_ulong,
            0x88ad9c8 as libc::c_ulong,
            0x49bbc2d1 as libc::c_ulong,
            0x8ae8effa as libc::c_ulong,
            0xcbd9f4e3 as libc::c_ulong,
            0xc4fb5ac as libc::c_ulong,
            0x4d7eaeb5 as libc::c_ulong,
            0x8e2d839e as libc::c_ulong,
            0xcf1c9887 as libc::c_ulong,
            0x5112c24a as libc::c_ulong,
            0x1023d953 as libc::c_ulong,
            0xd370f478 as libc::c_ulong,
            0x9241ef61 as libc::c_ulong,
            0x55d7ae2e as libc::c_ulong,
            0x14e6b537 as libc::c_ulong,
            0xd7b5981c as libc::c_ulong,
            0x96848305 as libc::c_ulong,
            0x59981b82 as libc::c_ulong,
            0x18a9009b as libc::c_ulong,
            0xdbfa2db0 as libc::c_ulong,
            0x9acb36a9 as libc::c_ulong,
            0x5d5d77e6 as libc::c_ulong,
            0x1c6c6cff as libc::c_ulong,
            0xdf3f41d4 as libc::c_ulong,
            0x9e0e5acd as libc::c_ulong,
            0xa2248495 as libc::c_ulong,
            0xe3159f8c as libc::c_ulong,
            0x2046b2a7 as libc::c_ulong,
            0x6177a9be as libc::c_ulong,
            0xa6e1e8f1 as libc::c_ulong,
            0xe7d0f3e8 as libc::c_ulong,
            0x2483dec3 as libc::c_ulong,
            0x65b2c5da as libc::c_ulong,
            0xaaae5d5d as libc::c_ulong,
            0xeb9f4644 as libc::c_ulong,
            0x28cc6b6f as libc::c_ulong,
            0x69fd7076 as libc::c_ulong,
            0xae6b3139 as libc::c_ulong,
            0xef5a2a20 as libc::c_ulong,
            0x2c09070b as libc::c_ulong,
            0x6d381c12 as libc::c_ulong,
            0xf33646df as libc::c_ulong,
            0xb2075dc6 as libc::c_ulong,
            0x715470ed as libc::c_ulong,
            0x30656bf4 as libc::c_ulong,
            0xf7f32abb as libc::c_ulong,
            0xb6c231a2 as libc::c_ulong,
            0x75911c89 as libc::c_ulong,
            0x34a00790 as libc::c_ulong,
            0xfbbc9f17 as libc::c_ulong,
            0xba8d840e as libc::c_ulong,
            0x79dea925 as libc::c_ulong,
            0x38efb23c as libc::c_ulong,
            0xff79f373 as libc::c_ulong,
            0xbe48e86a as libc::c_ulong,
            0x7d1bc541 as libc::c_ulong,
            0x3c2ade58 as libc::c_ulong,
            0x54f79f0 as libc::c_ulong,
            0x447e62e9 as libc::c_ulong,
            0x872d4fc2 as libc::c_ulong,
            0xc61c54db as libc::c_ulong,
            0x18a1594 as libc::c_ulong,
            0x40bb0e8d as libc::c_ulong,
            0x83e823a6 as libc::c_ulong,
            0xc2d938bf as libc::c_ulong,
            0xdc5a038 as libc::c_ulong,
            0x4cf4bb21 as libc::c_ulong,
            0x8fa7960a as libc::c_ulong,
            0xce968d13 as libc::c_ulong,
            0x900cc5c as libc::c_ulong,
            0x4831d745 as libc::c_ulong,
            0x8b62fa6e as libc::c_ulong,
            0xca53e177 as libc::c_ulong,
            0x545dbbba as libc::c_ulong,
            0x156ca0a3 as libc::c_ulong,
            0xd63f8d88 as libc::c_ulong,
            0x970e9691 as libc::c_ulong,
            0x5098d7de as libc::c_ulong,
            0x11a9ccc7 as libc::c_ulong,
            0xd2fae1ec as libc::c_ulong,
            0x93cbfaf5 as libc::c_ulong,
            0x5cd76272 as libc::c_ulong,
            0x1de6796b as libc::c_ulong,
            0xdeb55440 as libc::c_ulong,
            0x9f844f59 as libc::c_ulong,
            0x58120e16 as libc::c_ulong,
            0x1923150f as libc::c_ulong,
            0xda703824 as libc::c_ulong,
            0x9b41233d as libc::c_ulong,
            0xa76bfd65 as libc::c_ulong,
            0xe65ae67c as libc::c_ulong,
            0x2509cb57 as libc::c_ulong,
            0x6438d04e as libc::c_ulong,
            0xa3ae9101 as libc::c_ulong,
            0xe29f8a18 as libc::c_ulong,
            0x21cca733 as libc::c_ulong,
            0x60fdbc2a as libc::c_ulong,
            0xafe124ad as libc::c_ulong,
            0xeed03fb4 as libc::c_ulong,
            0x2d83129f as libc::c_ulong,
            0x6cb20986 as libc::c_ulong,
            0xab2448c9 as libc::c_ulong,
            0xea1553d0 as libc::c_ulong,
            0x29467efb as libc::c_ulong,
            0x687765e2 as libc::c_ulong,
            0xf6793f2f as libc::c_ulong,
            0xb7482436 as libc::c_ulong,
            0x741b091d as libc::c_ulong,
            0x352a1204 as libc::c_ulong,
            0xf2bc534b as libc::c_ulong,
            0xb38d4852 as libc::c_ulong,
            0x70de6579 as libc::c_ulong,
            0x31ef7e60 as libc::c_ulong,
            0xfef3e6e7 as libc::c_ulong,
            0xbfc2fdfe as libc::c_ulong,
            0x7c91d0d5 as libc::c_ulong,
            0x3da0cbcc as libc::c_ulong,
            0xfa368a83 as libc::c_ulong,
            0xbb07919a as libc::c_ulong,
            0x7854bcb1 as libc::c_ulong,
            0x3965a7a8 as libc::c_ulong,
            0x4b98833b as libc::c_ulong,
            0xaa99822 as libc::c_ulong,
            0xc9fab509 as libc::c_ulong,
            0x88cbae10 as libc::c_ulong,
            0x4f5def5f as libc::c_ulong,
            0xe6cf446 as libc::c_ulong,
            0xcd3fd96d as libc::c_ulong,
            0x8c0ec274 as libc::c_ulong,
            0x43125af3 as libc::c_ulong,
            0x22341ea as libc::c_ulong,
            0xc1706cc1 as libc::c_ulong,
            0x804177d8 as libc::c_ulong,
            0x47d73697 as libc::c_ulong,
            0x6e62d8e as libc::c_ulong,
            0xc5b500a5 as libc::c_ulong,
            0x84841bbc as libc::c_ulong,
            0x1a8a4171 as libc::c_ulong,
            0x5bbb5a68 as libc::c_ulong,
            0x98e87743 as libc::c_ulong,
            0xd9d96c5a as libc::c_ulong,
            0x1e4f2d15 as libc::c_ulong,
            0x5f7e360c as libc::c_ulong,
            0x9c2d1b27 as libc::c_ulong,
            0xdd1c003e as libc::c_ulong,
            0x120098b9 as libc::c_ulong,
            0x533183a0 as libc::c_ulong,
            0x9062ae8b as libc::c_ulong,
            0xd153b592 as libc::c_ulong,
            0x16c5f4dd as libc::c_ulong,
            0x57f4efc4 as libc::c_ulong,
            0x94a7c2ef as libc::c_ulong,
            0xd596d9f6 as libc::c_ulong,
            0xe9bc07ae as libc::c_ulong,
            0xa88d1cb7 as libc::c_ulong,
            0x6bde319c as libc::c_ulong,
            0x2aef2a85 as libc::c_ulong,
            0xed796bca as libc::c_ulong,
            0xac4870d3 as libc::c_ulong,
            0x6f1b5df8 as libc::c_ulong,
            0x2e2a46e1 as libc::c_ulong,
            0xe136de66 as libc::c_ulong,
            0xa007c57f as libc::c_ulong,
            0x6354e854 as libc::c_ulong,
            0x2265f34d as libc::c_ulong,
            0xe5f3b202 as libc::c_ulong,
            0xa4c2a91b as libc::c_ulong,
            0x67918430 as libc::c_ulong,
            0x26a09f29 as libc::c_ulong,
            0xb8aec5e4 as libc::c_ulong,
            0xf99fdefd as libc::c_ulong,
            0x3accf3d6 as libc::c_ulong,
            0x7bfde8cf as libc::c_ulong,
            0xbc6ba980 as libc::c_ulong,
            0xfd5ab299 as libc::c_ulong,
            0x3e099fb2 as libc::c_ulong,
            0x7f3884ab as libc::c_ulong,
            0xb0241c2c as libc::c_ulong,
            0xf1150735 as libc::c_ulong,
            0x32462a1e as libc::c_ulong,
            0x73773107 as libc::c_ulong,
            0xb4e17048 as libc::c_ulong,
            0xf5d06b51 as libc::c_ulong,
            0x3683467a as libc::c_ulong,
            0x77b25d63 as libc::c_ulong,
            0x4ed7facb as libc::c_ulong,
            0xfe6e1d2 as libc::c_ulong,
            0xccb5ccf9 as libc::c_ulong,
            0x8d84d7e0 as libc::c_ulong,
            0x4a1296af as libc::c_ulong,
            0xb238db6 as libc::c_ulong,
            0xc870a09d as libc::c_ulong,
            0x8941bb84 as libc::c_ulong,
            0x465d2303 as libc::c_ulong,
            0x76c381a as libc::c_ulong,
            0xc43f1531 as libc::c_ulong,
            0x850e0e28 as libc::c_ulong,
            0x42984f67 as libc::c_ulong,
            0x3a9547e as libc::c_ulong,
            0xc0fa7955 as libc::c_ulong,
            0x81cb624c as libc::c_ulong,
            0x1fc53881 as libc::c_ulong,
            0x5ef42398 as libc::c_ulong,
            0x9da70eb3 as libc::c_ulong,
            0xdc9615aa as libc::c_ulong,
            0x1b0054e5 as libc::c_ulong,
            0x5a314ffc as libc::c_ulong,
            0x996262d7 as libc::c_ulong,
            0xd85379ce as libc::c_ulong,
            0x174fe149 as libc::c_ulong,
            0x567efa50 as libc::c_ulong,
            0x952dd77b as libc::c_ulong,
            0xd41ccc62 as libc::c_ulong,
            0x138a8d2d as libc::c_ulong,
            0x52bb9634 as libc::c_ulong,
            0x91e8bb1f as libc::c_ulong,
            0xd0d9a006 as libc::c_ulong,
            0xecf37e5e as libc::c_ulong,
            0xadc26547 as libc::c_ulong,
            0x6e91486c as libc::c_ulong,
            0x2fa05375 as libc::c_ulong,
            0xe836123a as libc::c_ulong,
            0xa9070923 as libc::c_ulong,
            0x6a542408 as libc::c_ulong,
            0x2b653f11 as libc::c_ulong,
            0xe479a796 as libc::c_ulong,
            0xa548bc8f as libc::c_ulong,
            0x661b91a4 as libc::c_ulong,
            0x272a8abd as libc::c_ulong,
            0xe0bccbf2 as libc::c_ulong,
            0xa18dd0eb as libc::c_ulong,
            0x62defdc0 as libc::c_ulong,
            0x23efe6d9 as libc::c_ulong,
            0xbde1bc14 as libc::c_ulong,
            0xfcd0a70d as libc::c_ulong,
            0x3f838a26 as libc::c_ulong,
            0x7eb2913f as libc::c_ulong,
            0xb924d070 as libc::c_ulong,
            0xf815cb69 as libc::c_ulong,
            0x3b46e642 as libc::c_ulong,
            0x7a77fd5b as libc::c_ulong,
            0xb56b65dc as libc::c_ulong,
            0xf45a7ec5 as libc::c_ulong,
            0x370953ee as libc::c_ulong,
            0x763848f7 as libc::c_ulong,
            0xb1ae09b8 as libc::c_ulong,
            0xf09f12a1 as libc::c_ulong,
            0x33cc3f8a as libc::c_ulong,
            0x72fd2493 as libc::c_ulong,
        ],
        [
            0 as libc::c_ulong,
            0x376ac201 as libc::c_ulong,
            0x6ed48403 as libc::c_ulong,
            0x59be4602 as libc::c_ulong,
            0xdca80907 as libc::c_ulong,
            0xebc2cb06 as libc::c_ulong,
            0xb27c8d04 as libc::c_ulong,
            0x85164f05 as libc::c_ulong,
            0xb851130e as libc::c_ulong,
            0x8f3bd10f as libc::c_ulong,
            0xd685970d as libc::c_ulong,
            0xe1ef550c as libc::c_ulong,
            0x64f91a09 as libc::c_ulong,
            0x5393d808 as libc::c_ulong,
            0xa2d9e0a as libc::c_ulong,
            0x3d475c0b as libc::c_ulong,
            0x70a3261c as libc::c_ulong,
            0x47c9e41d as libc::c_ulong,
            0x1e77a21f as libc::c_ulong,
            0x291d601e as libc::c_ulong,
            0xac0b2f1b as libc::c_ulong,
            0x9b61ed1a as libc::c_ulong,
            0xc2dfab18 as libc::c_ulong,
            0xf5b56919 as libc::c_ulong,
            0xc8f23512 as libc::c_ulong,
            0xff98f713 as libc::c_ulong,
            0xa626b111 as libc::c_ulong,
            0x914c7310 as libc::c_ulong,
            0x145a3c15 as libc::c_ulong,
            0x2330fe14 as libc::c_ulong,
            0x7a8eb816 as libc::c_ulong,
            0x4de47a17 as libc::c_ulong,
            0xe0464d38 as libc::c_ulong,
            0xd72c8f39 as libc::c_ulong,
            0x8e92c93b as libc::c_ulong,
            0xb9f80b3a as libc::c_ulong,
            0x3cee443f as libc::c_ulong,
            0xb84863e as libc::c_ulong,
            0x523ac03c as libc::c_ulong,
            0x6550023d as libc::c_ulong,
            0x58175e36 as libc::c_ulong,
            0x6f7d9c37 as libc::c_ulong,
            0x36c3da35 as libc::c_ulong,
            0x1a91834 as libc::c_ulong,
            0x84bf5731 as libc::c_ulong,
            0xb3d59530 as libc::c_ulong,
            0xea6bd332 as libc::c_ulong,
            0xdd011133 as libc::c_ulong,
            0x90e56b24 as libc::c_ulong,
            0xa78fa925 as libc::c_ulong,
            0xfe31ef27 as libc::c_ulong,
            0xc95b2d26 as libc::c_ulong,
            0x4c4d6223 as libc::c_ulong,
            0x7b27a022 as libc::c_ulong,
            0x2299e620 as libc::c_ulong,
            0x15f32421 as libc::c_ulong,
            0x28b4782a as libc::c_ulong,
            0x1fdeba2b as libc::c_ulong,
            0x4660fc29 as libc::c_ulong,
            0x710a3e28 as libc::c_ulong,
            0xf41c712d as libc::c_ulong,
            0xc376b32c as libc::c_ulong,
            0x9ac8f52e as libc::c_ulong,
            0xada2372f as libc::c_ulong,
            0xc08d9a70 as libc::c_ulong,
            0xf7e75871 as libc::c_ulong,
            0xae591e73 as libc::c_ulong,
            0x9933dc72 as libc::c_ulong,
            0x1c259377 as libc::c_ulong,
            0x2b4f5176 as libc::c_ulong,
            0x72f11774 as libc::c_ulong,
            0x459bd575 as libc::c_ulong,
            0x78dc897e as libc::c_ulong,
            0x4fb64b7f as libc::c_ulong,
            0x16080d7d as libc::c_ulong,
            0x2162cf7c as libc::c_ulong,
            0xa4748079 as libc::c_ulong,
            0x931e4278 as libc::c_ulong,
            0xcaa0047a as libc::c_ulong,
            0xfdcac67b as libc::c_ulong,
            0xb02ebc6c as libc::c_ulong,
            0x87447e6d as libc::c_ulong,
            0xdefa386f as libc::c_ulong,
            0xe990fa6e as libc::c_ulong,
            0x6c86b56b as libc::c_ulong,
            0x5bec776a as libc::c_ulong,
            0x2523168 as libc::c_ulong,
            0x3538f369 as libc::c_ulong,
            0x87faf62 as libc::c_ulong,
            0x3f156d63 as libc::c_ulong,
            0x66ab2b61 as libc::c_ulong,
            0x51c1e960 as libc::c_ulong,
            0xd4d7a665 as libc::c_ulong,
            0xe3bd6464 as libc::c_ulong,
            0xba032266 as libc::c_ulong,
            0x8d69e067 as libc::c_ulong,
            0x20cbd748 as libc::c_ulong,
            0x17a11549 as libc::c_ulong,
            0x4e1f534b as libc::c_ulong,
            0x7975914a as libc::c_ulong,
            0xfc63de4f as libc::c_ulong,
            0xcb091c4e as libc::c_ulong,
            0x92b75a4c as libc::c_ulong,
            0xa5dd984d as libc::c_ulong,
            0x989ac446 as libc::c_ulong,
            0xaff00647 as libc::c_ulong,
            0xf64e4045 as libc::c_ulong,
            0xc1248244 as libc::c_ulong,
            0x4432cd41 as libc::c_ulong,
            0x73580f40 as libc::c_ulong,
            0x2ae64942 as libc::c_ulong,
            0x1d8c8b43 as libc::c_ulong,
            0x5068f154 as libc::c_ulong,
            0x67023355 as libc::c_ulong,
            0x3ebc7557 as libc::c_ulong,
            0x9d6b756 as libc::c_ulong,
            0x8cc0f853 as libc::c_ulong,
            0xbbaa3a52 as libc::c_ulong,
            0xe2147c50 as libc::c_ulong,
            0xd57ebe51 as libc::c_ulong,
            0xe839e25a as libc::c_ulong,
            0xdf53205b as libc::c_ulong,
            0x86ed6659 as libc::c_ulong,
            0xb187a458 as libc::c_ulong,
            0x3491eb5d as libc::c_ulong,
            0x3fb295c as libc::c_ulong,
            0x5a456f5e as libc::c_ulong,
            0x6d2fad5f as libc::c_ulong,
            0x801b35e1 as libc::c_ulong,
            0xb771f7e0 as libc::c_ulong,
            0xeecfb1e2 as libc::c_ulong,
            0xd9a573e3 as libc::c_ulong,
            0x5cb33ce6 as libc::c_ulong,
            0x6bd9fee7 as libc::c_ulong,
            0x3267b8e5 as libc::c_ulong,
            0x50d7ae4 as libc::c_ulong,
            0x384a26ef as libc::c_ulong,
            0xf20e4ee as libc::c_ulong,
            0x569ea2ec as libc::c_ulong,
            0x61f460ed as libc::c_ulong,
            0xe4e22fe8 as libc::c_ulong,
            0xd388ede9 as libc::c_ulong,
            0x8a36abeb as libc::c_ulong,
            0xbd5c69ea as libc::c_ulong,
            0xf0b813fd as libc::c_ulong,
            0xc7d2d1fc as libc::c_ulong,
            0x9e6c97fe as libc::c_ulong,
            0xa90655ff as libc::c_ulong,
            0x2c101afa as libc::c_ulong,
            0x1b7ad8fb as libc::c_ulong,
            0x42c49ef9 as libc::c_ulong,
            0x75ae5cf8 as libc::c_ulong,
            0x48e900f3 as libc::c_ulong,
            0x7f83c2f2 as libc::c_ulong,
            0x263d84f0 as libc::c_ulong,
            0x115746f1 as libc::c_ulong,
            0x944109f4 as libc::c_ulong,
            0xa32bcbf5 as libc::c_ulong,
            0xfa958df7 as libc::c_ulong,
            0xcdff4ff6 as libc::c_ulong,
            0x605d78d9 as libc::c_ulong,
            0x5737bad8 as libc::c_ulong,
            0xe89fcda as libc::c_ulong,
            0x39e33edb as libc::c_ulong,
            0xbcf571de as libc::c_ulong,
            0x8b9fb3df as libc::c_ulong,
            0xd221f5dd as libc::c_ulong,
            0xe54b37dc as libc::c_ulong,
            0xd80c6bd7 as libc::c_ulong,
            0xef66a9d6 as libc::c_ulong,
            0xb6d8efd4 as libc::c_ulong,
            0x81b22dd5 as libc::c_ulong,
            0x4a462d0 as libc::c_ulong,
            0x33cea0d1 as libc::c_ulong,
            0x6a70e6d3 as libc::c_ulong,
            0x5d1a24d2 as libc::c_ulong,
            0x10fe5ec5 as libc::c_ulong,
            0x27949cc4 as libc::c_ulong,
            0x7e2adac6 as libc::c_ulong,
            0x494018c7 as libc::c_ulong,
            0xcc5657c2 as libc::c_ulong,
            0xfb3c95c3 as libc::c_ulong,
            0xa282d3c1 as libc::c_ulong,
            0x95e811c0 as libc::c_ulong,
            0xa8af4dcb as libc::c_ulong,
            0x9fc58fca as libc::c_ulong,
            0xc67bc9c8 as libc::c_ulong,
            0xf1110bc9 as libc::c_ulong,
            0x740744cc as libc::c_ulong,
            0x436d86cd as libc::c_ulong,
            0x1ad3c0cf as libc::c_ulong,
            0x2db902ce as libc::c_ulong,
            0x4096af91 as libc::c_ulong,
            0x77fc6d90 as libc::c_ulong,
            0x2e422b92 as libc::c_ulong,
            0x1928e993 as libc::c_ulong,
            0x9c3ea696 as libc::c_ulong,
            0xab546497 as libc::c_ulong,
            0xf2ea2295 as libc::c_ulong,
            0xc580e094 as libc::c_ulong,
            0xf8c7bc9f as libc::c_ulong,
            0xcfad7e9e as libc::c_ulong,
            0x9613389c as libc::c_ulong,
            0xa179fa9d as libc::c_ulong,
            0x246fb598 as libc::c_ulong,
            0x13057799 as libc::c_ulong,
            0x4abb319b as libc::c_ulong,
            0x7dd1f39a as libc::c_ulong,
            0x3035898d as libc::c_ulong,
            0x75f4b8c as libc::c_ulong,
            0x5ee10d8e as libc::c_ulong,
            0x698bcf8f as libc::c_ulong,
            0xec9d808a as libc::c_ulong,
            0xdbf7428b as libc::c_ulong,
            0x82490489 as libc::c_ulong,
            0xb523c688 as libc::c_ulong,
            0x88649a83 as libc::c_ulong,
            0xbf0e5882 as libc::c_ulong,
            0xe6b01e80 as libc::c_ulong,
            0xd1dadc81 as libc::c_ulong,
            0x54cc9384 as libc::c_ulong,
            0x63a65185 as libc::c_ulong,
            0x3a181787 as libc::c_ulong,
            0xd72d586 as libc::c_ulong,
            0xa0d0e2a9 as libc::c_ulong,
            0x97ba20a8 as libc::c_ulong,
            0xce0466aa as libc::c_ulong,
            0xf96ea4ab as libc::c_ulong,
            0x7c78ebae as libc::c_ulong,
            0x4b1229af as libc::c_ulong,
            0x12ac6fad as libc::c_ulong,
            0x25c6adac as libc::c_ulong,
            0x1881f1a7 as libc::c_ulong,
            0x2feb33a6 as libc::c_ulong,
            0x765575a4 as libc::c_ulong,
            0x413fb7a5 as libc::c_ulong,
            0xc429f8a0 as libc::c_ulong,
            0xf3433aa1 as libc::c_ulong,
            0xaafd7ca3 as libc::c_ulong,
            0x9d97bea2 as libc::c_ulong,
            0xd073c4b5 as libc::c_ulong,
            0xe71906b4 as libc::c_ulong,
            0xbea740b6 as libc::c_ulong,
            0x89cd82b7 as libc::c_ulong,
            0xcdbcdb2 as libc::c_ulong,
            0x3bb10fb3 as libc::c_ulong,
            0x620f49b1 as libc::c_ulong,
            0x55658bb0 as libc::c_ulong,
            0x6822d7bb as libc::c_ulong,
            0x5f4815ba as libc::c_ulong,
            0x6f653b8 as libc::c_ulong,
            0x319c91b9 as libc::c_ulong,
            0xb48adebc as libc::c_ulong,
            0x83e01cbd as libc::c_ulong,
            0xda5e5abf as libc::c_ulong,
            0xed3498be as libc::c_ulong,
        ],
        [
            0 as libc::c_ulong,
            0x6567bcb8 as libc::c_ulong,
            0x8bc809aa as libc::c_ulong,
            0xeeafb512 as libc::c_ulong,
            0x5797628f as libc::c_ulong,
            0x32f0de37 as libc::c_ulong,
            0xdc5f6b25 as libc::c_ulong,
            0xb938d79d as libc::c_ulong,
            0xef28b4c5 as libc::c_ulong,
            0x8a4f087d as libc::c_ulong,
            0x64e0bd6f as libc::c_ulong,
            0x18701d7 as libc::c_ulong,
            0xb8bfd64a as libc::c_ulong,
            0xddd86af2 as libc::c_ulong,
            0x3377dfe0 as libc::c_ulong,
            0x56106358 as libc::c_ulong,
            0x9f571950 as libc::c_ulong,
            0xfa30a5e8 as libc::c_ulong,
            0x149f10fa as libc::c_ulong,
            0x71f8ac42 as libc::c_ulong,
            0xc8c07bdf as libc::c_ulong,
            0xada7c767 as libc::c_ulong,
            0x43087275 as libc::c_ulong,
            0x266fcecd as libc::c_ulong,
            0x707fad95 as libc::c_ulong,
            0x1518112d as libc::c_ulong,
            0xfbb7a43f as libc::c_ulong,
            0x9ed01887 as libc::c_ulong,
            0x27e8cf1a as libc::c_ulong,
            0x428f73a2 as libc::c_ulong,
            0xac20c6b0 as libc::c_ulong,
            0xc9477a08 as libc::c_ulong,
            0x3eaf32a0 as libc::c_ulong,
            0x5bc88e18 as libc::c_ulong,
            0xb5673b0a as libc::c_ulong,
            0xd00087b2 as libc::c_ulong,
            0x6938502f as libc::c_ulong,
            0xc5fec97 as libc::c_ulong,
            0xe2f05985 as libc::c_ulong,
            0x8797e53d as libc::c_ulong,
            0xd1878665 as libc::c_ulong,
            0xb4e03add as libc::c_ulong,
            0x5a4f8fcf as libc::c_ulong,
            0x3f283377 as libc::c_ulong,
            0x8610e4ea as libc::c_ulong,
            0xe3775852 as libc::c_ulong,
            0xdd8ed40 as libc::c_ulong,
            0x68bf51f8 as libc::c_ulong,
            0xa1f82bf0 as libc::c_ulong,
            0xc49f9748 as libc::c_ulong,
            0x2a30225a as libc::c_ulong,
            0x4f579ee2 as libc::c_ulong,
            0xf66f497f as libc::c_ulong,
            0x9308f5c7 as libc::c_ulong,
            0x7da740d5 as libc::c_ulong,
            0x18c0fc6d as libc::c_ulong,
            0x4ed09f35 as libc::c_ulong,
            0x2bb7238d as libc::c_ulong,
            0xc518969f as libc::c_ulong,
            0xa07f2a27 as libc::c_ulong,
            0x1947fdba as libc::c_ulong,
            0x7c204102 as libc::c_ulong,
            0x928ff410 as libc::c_ulong,
            0xf7e848a8 as libc::c_ulong,
            0x3d58149b as libc::c_ulong,
            0x583fa823 as libc::c_ulong,
            0xb6901d31 as libc::c_ulong,
            0xd3f7a189 as libc::c_ulong,
            0x6acf7614 as libc::c_ulong,
            0xfa8caac as libc::c_ulong,
            0xe1077fbe as libc::c_ulong,
            0x8460c306 as libc::c_ulong,
            0xd270a05e as libc::c_ulong,
            0xb7171ce6 as libc::c_ulong,
            0x59b8a9f4 as libc::c_ulong,
            0x3cdf154c as libc::c_ulong,
            0x85e7c2d1 as libc::c_ulong,
            0xe0807e69 as libc::c_ulong,
            0xe2fcb7b as libc::c_ulong,
            0x6b4877c3 as libc::c_ulong,
            0xa20f0dcb as libc::c_ulong,
            0xc768b173 as libc::c_ulong,
            0x29c70461 as libc::c_ulong,
            0x4ca0b8d9 as libc::c_ulong,
            0xf5986f44 as libc::c_ulong,
            0x90ffd3fc as libc::c_ulong,
            0x7e5066ee as libc::c_ulong,
            0x1b37da56 as libc::c_ulong,
            0x4d27b90e as libc::c_ulong,
            0x284005b6 as libc::c_ulong,
            0xc6efb0a4 as libc::c_ulong,
            0xa3880c1c as libc::c_ulong,
            0x1ab0db81 as libc::c_ulong,
            0x7fd76739 as libc::c_ulong,
            0x9178d22b as libc::c_ulong,
            0xf41f6e93 as libc::c_ulong,
            0x3f7263b as libc::c_ulong,
            0x66909a83 as libc::c_ulong,
            0x883f2f91 as libc::c_ulong,
            0xed589329 as libc::c_ulong,
            0x546044b4 as libc::c_ulong,
            0x3107f80c as libc::c_ulong,
            0xdfa84d1e as libc::c_ulong,
            0xbacff1a6 as libc::c_ulong,
            0xecdf92fe as libc::c_ulong,
            0x89b82e46 as libc::c_ulong,
            0x67179b54 as libc::c_ulong,
            0x27027ec as libc::c_ulong,
            0xbb48f071 as libc::c_ulong,
            0xde2f4cc9 as libc::c_ulong,
            0x3080f9db as libc::c_ulong,
            0x55e74563 as libc::c_ulong,
            0x9ca03f6b as libc::c_ulong,
            0xf9c783d3 as libc::c_ulong,
            0x176836c1 as libc::c_ulong,
            0x720f8a79 as libc::c_ulong,
            0xcb375de4 as libc::c_ulong,
            0xae50e15c as libc::c_ulong,
            0x40ff544e as libc::c_ulong,
            0x2598e8f6 as libc::c_ulong,
            0x73888bae as libc::c_ulong,
            0x16ef3716 as libc::c_ulong,
            0xf8408204 as libc::c_ulong,
            0x9d273ebc as libc::c_ulong,
            0x241fe921 as libc::c_ulong,
            0x41785599 as libc::c_ulong,
            0xafd7e08b as libc::c_ulong,
            0xcab05c33 as libc::c_ulong,
            0x3bb659ed as libc::c_ulong,
            0x5ed1e555 as libc::c_ulong,
            0xb07e5047 as libc::c_ulong,
            0xd519ecff as libc::c_ulong,
            0x6c213b62 as libc::c_ulong,
            0x94687da as libc::c_ulong,
            0xe7e932c8 as libc::c_ulong,
            0x828e8e70 as libc::c_ulong,
            0xd49eed28 as libc::c_ulong,
            0xb1f95190 as libc::c_ulong,
            0x5f56e482 as libc::c_ulong,
            0x3a31583a as libc::c_ulong,
            0x83098fa7 as libc::c_ulong,
            0xe66e331f as libc::c_ulong,
            0x8c1860d as libc::c_ulong,
            0x6da63ab5 as libc::c_ulong,
            0xa4e140bd as libc::c_ulong,
            0xc186fc05 as libc::c_ulong,
            0x2f294917 as libc::c_ulong,
            0x4a4ef5af as libc::c_ulong,
            0xf3762232 as libc::c_ulong,
            0x96119e8a as libc::c_ulong,
            0x78be2b98 as libc::c_ulong,
            0x1dd99720 as libc::c_ulong,
            0x4bc9f478 as libc::c_ulong,
            0x2eae48c0 as libc::c_ulong,
            0xc001fdd2 as libc::c_ulong,
            0xa566416a as libc::c_ulong,
            0x1c5e96f7 as libc::c_ulong,
            0x79392a4f as libc::c_ulong,
            0x97969f5d as libc::c_ulong,
            0xf2f123e5 as libc::c_ulong,
            0x5196b4d as libc::c_ulong,
            0x607ed7f5 as libc::c_ulong,
            0x8ed162e7 as libc::c_ulong,
            0xebb6de5f as libc::c_ulong,
            0x528e09c2 as libc::c_ulong,
            0x37e9b57a as libc::c_ulong,
            0xd9460068 as libc::c_ulong,
            0xbc21bcd0 as libc::c_ulong,
            0xea31df88 as libc::c_ulong,
            0x8f566330 as libc::c_ulong,
            0x61f9d622 as libc::c_ulong,
            0x49e6a9a as libc::c_ulong,
            0xbda6bd07 as libc::c_ulong,
            0xd8c101bf as libc::c_ulong,
            0x366eb4ad as libc::c_ulong,
            0x53090815 as libc::c_ulong,
            0x9a4e721d as libc::c_ulong,
            0xff29cea5 as libc::c_ulong,
            0x11867bb7 as libc::c_ulong,
            0x74e1c70f as libc::c_ulong,
            0xcdd91092 as libc::c_ulong,
            0xa8beac2a as libc::c_ulong,
            0x46111938 as libc::c_ulong,
            0x2376a580 as libc::c_ulong,
            0x7566c6d8 as libc::c_ulong,
            0x10017a60 as libc::c_ulong,
            0xfeaecf72 as libc::c_ulong,
            0x9bc973ca as libc::c_ulong,
            0x22f1a457 as libc::c_ulong,
            0x479618ef as libc::c_ulong,
            0xa939adfd as libc::c_ulong,
            0xcc5e1145 as libc::c_ulong,
            0x6ee4d76 as libc::c_ulong,
            0x6389f1ce as libc::c_ulong,
            0x8d2644dc as libc::c_ulong,
            0xe841f864 as libc::c_ulong,
            0x51792ff9 as libc::c_ulong,
            0x341e9341 as libc::c_ulong,
            0xdab12653 as libc::c_ulong,
            0xbfd69aeb as libc::c_ulong,
            0xe9c6f9b3 as libc::c_ulong,
            0x8ca1450b as libc::c_ulong,
            0x620ef019 as libc::c_ulong,
            0x7694ca1 as libc::c_ulong,
            0xbe519b3c as libc::c_ulong,
            0xdb362784 as libc::c_ulong,
            0x35999296 as libc::c_ulong,
            0x50fe2e2e as libc::c_ulong,
            0x99b95426 as libc::c_ulong,
            0xfcdee89e as libc::c_ulong,
            0x12715d8c as libc::c_ulong,
            0x7716e134 as libc::c_ulong,
            0xce2e36a9 as libc::c_ulong,
            0xab498a11 as libc::c_ulong,
            0x45e63f03 as libc::c_ulong,
            0x208183bb as libc::c_ulong,
            0x7691e0e3 as libc::c_ulong,
            0x13f65c5b as libc::c_ulong,
            0xfd59e949 as libc::c_ulong,
            0x983e55f1 as libc::c_ulong,
            0x2106826c as libc::c_ulong,
            0x44613ed4 as libc::c_ulong,
            0xaace8bc6 as libc::c_ulong,
            0xcfa9377e as libc::c_ulong,
            0x38417fd6 as libc::c_ulong,
            0x5d26c36e as libc::c_ulong,
            0xb389767c as libc::c_ulong,
            0xd6eecac4 as libc::c_ulong,
            0x6fd61d59 as libc::c_ulong,
            0xab1a1e1 as libc::c_ulong,
            0xe41e14f3 as libc::c_ulong,
            0x8179a84b as libc::c_ulong,
            0xd769cb13 as libc::c_ulong,
            0xb20e77ab as libc::c_ulong,
            0x5ca1c2b9 as libc::c_ulong,
            0x39c67e01 as libc::c_ulong,
            0x80fea99c as libc::c_ulong,
            0xe5991524 as libc::c_ulong,
            0xb36a036 as libc::c_ulong,
            0x6e511c8e as libc::c_ulong,
            0xa7166686 as libc::c_ulong,
            0xc271da3e as libc::c_ulong,
            0x2cde6f2c as libc::c_ulong,
            0x49b9d394 as libc::c_ulong,
            0xf0810409 as libc::c_ulong,
            0x95e6b8b1 as libc::c_ulong,
            0x7b490da3 as libc::c_ulong,
            0x1e2eb11b as libc::c_ulong,
            0x483ed243 as libc::c_ulong,
            0x2d596efb as libc::c_ulong,
            0xc3f6dbe9 as libc::c_ulong,
            0xa6916751 as libc::c_ulong,
            0x1fa9b0cc as libc::c_ulong,
            0x7ace0c74 as libc::c_ulong,
            0x9461b966 as libc::c_ulong,
            0xf10605de as libc::c_ulong,
        ],
    ];
}
pub use crate::src::zlib::crc32::crc32_h::crc_table;
pub use crate::stddef_h::ptrdiff_t;
pub use crate::stdlib::__off_t;
pub use crate::stdlib::off_t;
pub use crate::zconf_h::uInt;
pub use crate::zconf_h::uLong;
pub use crate::zconf_h::uLongf;
pub use crate::zconf_h::Byte;
pub use crate::zconf_h::Bytef;
/* crc32.c -- compute the CRC-32 of a data stream
 * Copyright (C) 1995-2005 Mark Adler
 * For conditions of distribution and use, see copyright notice in zlib.h
 *
 * Thanks to Rodney Brown <rbrown64@csc.com.au> for his contribution of faster
 * CRC methods: exclusive-oring 32 bits of data at a time, and pre-computing
 * tables for updating the shift register in one step with three exclusive-ors
 * instead of four steps with four exclusive-ors.  This results in about a
 * factor of two increase in speed on a Power PC G4 (PPC7455) using gcc -O3.
 */
/* @(#) $Id$ */
/*
 Note on the use of DYNAMIC_CRC_TABLE: there is no mutex or semaphore
 protection on the static variables used to control the first-use generation
 of the crc tables.  Therefore, if you #define DYNAMIC_CRC_TABLE, you should
 first call get_crc_table() to initialize the tables before allowing more than
 one thread to use crc32().
*/
/* MAKECRCH */
/* Find a four-byte integer type for crc32_little() and crc32_big(). */
/* need ANSI C limits.h to determine sizes */

pub type u4 = u32;
/*
     Combine two CRC-32 check values into one.  For two sequences of bytes,
   seq1 and seq2 with lengths len1 and len2, CRC-32 check values were
   calculated for each, crc1 and crc2.  crc32_combine() returns the CRC-32
   check value of seq1 and seq2 concatenated, requiring only crc1, crc2, and
   len2.
*/
/* various hacks, don't look :) */
/* deflateInit and inflateInit are macros to allow checking the zlib version
 * and the compiler's view of z_stream:
 */
/* !DYNAMIC_CRC_TABLE */
/* ========================================================================
 * Tables of CRC-32s of all single-byte values, made by make_crc_table().
 */
/* DYNAMIC_CRC_TABLE */
/* =========================================================================
 * This function can be used by asm versions of crc32()
 */
#[no_mangle]

pub unsafe extern "C" fn get_crc_table() -> *const uLongf {
    /* DYNAMIC_CRC_TABLE */
    return crc_table.as_ptr() as *const libc::c_ulong;
}
/* ========================================================================= */
/* ========================================================================= */
#[no_mangle]

pub unsafe extern "C" fn crc32(
    mut crc: libc::c_ulong,
    mut buf: *const u8,
    mut len: u32,
) -> uLong {
    if buf.is_null() {
        return 0 as libc::c_ulong;
    }
    /* DYNAMIC_CRC_TABLE */
    if ::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong
        == ::std::mem::size_of::<ptrdiff_t>() as libc::c_ulong
    {
        let mut endian: u4 = 0;
        endian = 1 as i32 as u4;
        if *(&mut endian as *mut u4 as *mut u8) != 0 {
            return crc32_little(crc, buf, len);
        } else {
            return crc32_big(crc, buf, len);
        }
    }
    /* BYFOUR */
    crc = crc ^ 0xffffffff as libc::c_ulong;
    while len >= 8 as i32 as u32 {
        let fresh0 = buf;
        buf = buf.offset(1);
        crc = crc_table[0 as i32 as usize][((crc as i32 ^ *fresh0 as i32) & 0xff as i32) as usize]
            ^ crc >> 8 as i32;
        let fresh1 = buf;
        buf = buf.offset(1);
        crc = crc_table[0 as i32 as usize][((crc as i32 ^ *fresh1 as i32) & 0xff as i32) as usize]
            ^ crc >> 8 as i32;
        let fresh2 = buf;
        buf = buf.offset(1);
        crc = crc_table[0 as i32 as usize][((crc as i32 ^ *fresh2 as i32) & 0xff as i32) as usize]
            ^ crc >> 8 as i32;
        let fresh3 = buf;
        buf = buf.offset(1);
        crc = crc_table[0 as i32 as usize][((crc as i32 ^ *fresh3 as i32) & 0xff as i32) as usize]
            ^ crc >> 8 as i32;
        let fresh4 = buf;
        buf = buf.offset(1);
        crc = crc_table[0 as i32 as usize][((crc as i32 ^ *fresh4 as i32) & 0xff as i32) as usize]
            ^ crc >> 8 as i32;
        let fresh5 = buf;
        buf = buf.offset(1);
        crc = crc_table[0 as i32 as usize][((crc as i32 ^ *fresh5 as i32) & 0xff as i32) as usize]
            ^ crc >> 8 as i32;
        let fresh6 = buf;
        buf = buf.offset(1);
        crc = crc_table[0 as i32 as usize][((crc as i32 ^ *fresh6 as i32) & 0xff as i32) as usize]
            ^ crc >> 8 as i32;
        let fresh7 = buf;
        buf = buf.offset(1);
        crc = crc_table[0 as i32 as usize][((crc as i32 ^ *fresh7 as i32) & 0xff as i32) as usize]
            ^ crc >> 8 as i32;
        len = len.wrapping_sub(8 as i32 as u32)
    }
    if len != 0 {
        loop {
            let fresh8 = buf;
            buf = buf.offset(1);
            crc = crc_table[0 as i32 as usize]
                [((crc as i32 ^ *fresh8 as i32) & 0xff as i32) as usize]
                ^ crc >> 8 as i32;
            len = len.wrapping_sub(1);
            if !(len != 0) {
                break;
            }
        }
    }
    return crc ^ 0xffffffff as libc::c_ulong;
}
/* STDC */
/* !NOBYFOUR */
/* Definitions for doing the crc four data bytes at a time. */
/* ========================================================================= */
/* ========================================================================= */

unsafe extern "C" fn crc32_little(
    mut crc: libc::c_ulong,
    mut buf: *const u8,
    mut len: u32,
) -> libc::c_ulong {
    let mut c: u4 = 0;
    let mut buf4: *const u4 = 0 as *const u4;
    c = crc as u4;
    c = !c;
    while len != 0 && buf as ptrdiff_t & 3 as i32 as isize != 0 {
        let fresh9 = buf;
        buf = buf.offset(1);
        c = (crc_table[0 as i32 as usize][((c ^ *fresh9 as u32) & 0xff as i32 as u32) as usize]
            ^ (c >> 8 as i32) as libc::c_ulong) as u4;
        len = len.wrapping_sub(1)
    }
    buf4 = buf as *const libc::c_void as *const u4;
    while len >= 32 as i32 as u32 {
        let fresh10 = buf4;
        buf4 = buf4.offset(1);
        c ^= *fresh10;
        c = (crc_table[3 as i32 as usize][(c & 0xff as i32 as u32) as usize]
            ^ crc_table[2 as i32 as usize][(c >> 8 as i32 & 0xff as i32 as u32) as usize]
            ^ crc_table[1 as i32 as usize][(c >> 16 as i32 & 0xff as i32 as u32) as usize]
            ^ crc_table[0 as i32 as usize][(c >> 24 as i32) as usize]) as u4;
        let fresh11 = buf4;
        buf4 = buf4.offset(1);
        c ^= *fresh11;
        c = (crc_table[3 as i32 as usize][(c & 0xff as i32 as u32) as usize]
            ^ crc_table[2 as i32 as usize][(c >> 8 as i32 & 0xff as i32 as u32) as usize]
            ^ crc_table[1 as i32 as usize][(c >> 16 as i32 & 0xff as i32 as u32) as usize]
            ^ crc_table[0 as i32 as usize][(c >> 24 as i32) as usize]) as u4;
        let fresh12 = buf4;
        buf4 = buf4.offset(1);
        c ^= *fresh12;
        c = (crc_table[3 as i32 as usize][(c & 0xff as i32 as u32) as usize]
            ^ crc_table[2 as i32 as usize][(c >> 8 as i32 & 0xff as i32 as u32) as usize]
            ^ crc_table[1 as i32 as usize][(c >> 16 as i32 & 0xff as i32 as u32) as usize]
            ^ crc_table[0 as i32 as usize][(c >> 24 as i32) as usize]) as u4;
        let fresh13 = buf4;
        buf4 = buf4.offset(1);
        c ^= *fresh13;
        c = (crc_table[3 as i32 as usize][(c & 0xff as i32 as u32) as usize]
            ^ crc_table[2 as i32 as usize][(c >> 8 as i32 & 0xff as i32 as u32) as usize]
            ^ crc_table[1 as i32 as usize][(c >> 16 as i32 & 0xff as i32 as u32) as usize]
            ^ crc_table[0 as i32 as usize][(c >> 24 as i32) as usize]) as u4;
        let fresh14 = buf4;
        buf4 = buf4.offset(1);
        c ^= *fresh14;
        c = (crc_table[3 as i32 as usize][(c & 0xff as i32 as u32) as usize]
            ^ crc_table[2 as i32 as usize][(c >> 8 as i32 & 0xff as i32 as u32) as usize]
            ^ crc_table[1 as i32 as usize][(c >> 16 as i32 & 0xff as i32 as u32) as usize]
            ^ crc_table[0 as i32 as usize][(c >> 24 as i32) as usize]) as u4;
        let fresh15 = buf4;
        buf4 = buf4.offset(1);
        c ^= *fresh15;
        c = (crc_table[3 as i32 as usize][(c & 0xff as i32 as u32) as usize]
            ^ crc_table[2 as i32 as usize][(c >> 8 as i32 & 0xff as i32 as u32) as usize]
            ^ crc_table[1 as i32 as usize][(c >> 16 as i32 & 0xff as i32 as u32) as usize]
            ^ crc_table[0 as i32 as usize][(c >> 24 as i32) as usize]) as u4;
        let fresh16 = buf4;
        buf4 = buf4.offset(1);
        c ^= *fresh16;
        c = (crc_table[3 as i32 as usize][(c & 0xff as i32 as u32) as usize]
            ^ crc_table[2 as i32 as usize][(c >> 8 as i32 & 0xff as i32 as u32) as usize]
            ^ crc_table[1 as i32 as usize][(c >> 16 as i32 & 0xff as i32 as u32) as usize]
            ^ crc_table[0 as i32 as usize][(c >> 24 as i32) as usize]) as u4;
        let fresh17 = buf4;
        buf4 = buf4.offset(1);
        c ^= *fresh17;
        c = (crc_table[3 as i32 as usize][(c & 0xff as i32 as u32) as usize]
            ^ crc_table[2 as i32 as usize][(c >> 8 as i32 & 0xff as i32 as u32) as usize]
            ^ crc_table[1 as i32 as usize][(c >> 16 as i32 & 0xff as i32 as u32) as usize]
            ^ crc_table[0 as i32 as usize][(c >> 24 as i32) as usize]) as u4;
        len = len.wrapping_sub(32 as i32 as u32)
    }
    while len >= 4 as i32 as u32 {
        let fresh18 = buf4;
        buf4 = buf4.offset(1);
        c ^= *fresh18;
        c = (crc_table[3 as i32 as usize][(c & 0xff as i32 as u32) as usize]
            ^ crc_table[2 as i32 as usize][(c >> 8 as i32 & 0xff as i32 as u32) as usize]
            ^ crc_table[1 as i32 as usize][(c >> 16 as i32 & 0xff as i32 as u32) as usize]
            ^ crc_table[0 as i32 as usize][(c >> 24 as i32) as usize]) as u4;
        len = len.wrapping_sub(4 as i32 as u32)
    }
    buf = buf4 as *const u8;
    if len != 0 {
        loop {
            let fresh19 = buf;
            buf = buf.offset(1);
            c = (crc_table[0 as i32 as usize]
                [((c ^ *fresh19 as u32) & 0xff as i32 as u32) as usize]
                ^ (c >> 8 as i32) as libc::c_ulong) as u4;
            len = len.wrapping_sub(1);
            if !(len != 0) {
                break;
            }
        }
    }
    c = !c;
    return c as libc::c_ulong;
}
/* ========================================================================= */
/* ========================================================================= */

unsafe extern "C" fn crc32_big(
    mut crc: libc::c_ulong,
    mut buf: *const u8,
    mut len: u32,
) -> libc::c_ulong {
    let mut c: u4 = 0;
    let mut buf4: *const u4 = 0 as *const u4;
    c = (crc as u4 >> 24 as i32)
        .wrapping_add(crc as u4 >> 8 as i32 & 0xff00 as i32 as u32)
        .wrapping_add((crc as u4 & 0xff00 as i32 as u32) << 8 as i32)
        .wrapping_add((crc as u4 & 0xff as i32 as u32) << 24 as i32);
    c = !c;
    while len != 0 && buf as ptrdiff_t & 3 as i32 as isize != 0 {
        let fresh20 = buf;
        buf = buf.offset(1);
        c = (crc_table[4 as i32 as usize][(c >> 24 as i32 ^ *fresh20 as u32) as usize]
            ^ (c << 8 as i32) as libc::c_ulong) as u4;
        len = len.wrapping_sub(1)
    }
    buf4 = buf as *const libc::c_void as *const u4;
    buf4 = buf4.offset(-1);
    while len >= 32 as i32 as u32 {
        buf4 = buf4.offset(1);
        c ^= *buf4;
        c = (crc_table[4 as i32 as usize][(c & 0xff as i32 as u32) as usize]
            ^ crc_table[5 as i32 as usize][(c >> 8 as i32 & 0xff as i32 as u32) as usize]
            ^ crc_table[6 as i32 as usize][(c >> 16 as i32 & 0xff as i32 as u32) as usize]
            ^ crc_table[7 as i32 as usize][(c >> 24 as i32) as usize]) as u4;
        buf4 = buf4.offset(1);
        c ^= *buf4;
        c = (crc_table[4 as i32 as usize][(c & 0xff as i32 as u32) as usize]
            ^ crc_table[5 as i32 as usize][(c >> 8 as i32 & 0xff as i32 as u32) as usize]
            ^ crc_table[6 as i32 as usize][(c >> 16 as i32 & 0xff as i32 as u32) as usize]
            ^ crc_table[7 as i32 as usize][(c >> 24 as i32) as usize]) as u4;
        buf4 = buf4.offset(1);
        c ^= *buf4;
        c = (crc_table[4 as i32 as usize][(c & 0xff as i32 as u32) as usize]
            ^ crc_table[5 as i32 as usize][(c >> 8 as i32 & 0xff as i32 as u32) as usize]
            ^ crc_table[6 as i32 as usize][(c >> 16 as i32 & 0xff as i32 as u32) as usize]
            ^ crc_table[7 as i32 as usize][(c >> 24 as i32) as usize]) as u4;
        buf4 = buf4.offset(1);
        c ^= *buf4;
        c = (crc_table[4 as i32 as usize][(c & 0xff as i32 as u32) as usize]
            ^ crc_table[5 as i32 as usize][(c >> 8 as i32 & 0xff as i32 as u32) as usize]
            ^ crc_table[6 as i32 as usize][(c >> 16 as i32 & 0xff as i32 as u32) as usize]
            ^ crc_table[7 as i32 as usize][(c >> 24 as i32) as usize]) as u4;
        buf4 = buf4.offset(1);
        c ^= *buf4;
        c = (crc_table[4 as i32 as usize][(c & 0xff as i32 as u32) as usize]
            ^ crc_table[5 as i32 as usize][(c >> 8 as i32 & 0xff as i32 as u32) as usize]
            ^ crc_table[6 as i32 as usize][(c >> 16 as i32 & 0xff as i32 as u32) as usize]
            ^ crc_table[7 as i32 as usize][(c >> 24 as i32) as usize]) as u4;
        buf4 = buf4.offset(1);
        c ^= *buf4;
        c = (crc_table[4 as i32 as usize][(c & 0xff as i32 as u32) as usize]
            ^ crc_table[5 as i32 as usize][(c >> 8 as i32 & 0xff as i32 as u32) as usize]
            ^ crc_table[6 as i32 as usize][(c >> 16 as i32 & 0xff as i32 as u32) as usize]
            ^ crc_table[7 as i32 as usize][(c >> 24 as i32) as usize]) as u4;
        buf4 = buf4.offset(1);
        c ^= *buf4;
        c = (crc_table[4 as i32 as usize][(c & 0xff as i32 as u32) as usize]
            ^ crc_table[5 as i32 as usize][(c >> 8 as i32 & 0xff as i32 as u32) as usize]
            ^ crc_table[6 as i32 as usize][(c >> 16 as i32 & 0xff as i32 as u32) as usize]
            ^ crc_table[7 as i32 as usize][(c >> 24 as i32) as usize]) as u4;
        buf4 = buf4.offset(1);
        c ^= *buf4;
        c = (crc_table[4 as i32 as usize][(c & 0xff as i32 as u32) as usize]
            ^ crc_table[5 as i32 as usize][(c >> 8 as i32 & 0xff as i32 as u32) as usize]
            ^ crc_table[6 as i32 as usize][(c >> 16 as i32 & 0xff as i32 as u32) as usize]
            ^ crc_table[7 as i32 as usize][(c >> 24 as i32) as usize]) as u4;
        len = len.wrapping_sub(32 as i32 as u32)
    }
    while len >= 4 as i32 as u32 {
        buf4 = buf4.offset(1);
        c ^= *buf4;
        c = (crc_table[4 as i32 as usize][(c & 0xff as i32 as u32) as usize]
            ^ crc_table[5 as i32 as usize][(c >> 8 as i32 & 0xff as i32 as u32) as usize]
            ^ crc_table[6 as i32 as usize][(c >> 16 as i32 & 0xff as i32 as u32) as usize]
            ^ crc_table[7 as i32 as usize][(c >> 24 as i32) as usize]) as u4;
        len = len.wrapping_sub(4 as i32 as u32)
    }
    buf4 = buf4.offset(1);
    buf = buf4 as *const u8;
    if len != 0 {
        loop {
            let fresh21 = buf;
            buf = buf.offset(1);
            c = (crc_table[4 as i32 as usize][(c >> 24 as i32 ^ *fresh21 as u32) as usize]
                ^ (c << 8 as i32) as libc::c_ulong) as u4;
            len = len.wrapping_sub(1);
            if !(len != 0) {
                break;
            }
        }
    }
    c = !c;
    return (c >> 24 as i32)
        .wrapping_add(c >> 8 as i32 & 0xff00 as i32 as u32)
        .wrapping_add((c & 0xff00 as i32 as u32) << 8 as i32)
        .wrapping_add((c & 0xff as i32 as u32) << 24 as i32) as libc::c_ulong;
}
/* BYFOUR */
/* Local functions for crc concatenation */
/* dimension of GF(2) vectors (length of CRC) */
/* ========================================================================= */

unsafe extern "C" fn gf2_matrix_times(
    mut mat: *mut libc::c_ulong,
    mut vec: libc::c_ulong,
) -> libc::c_ulong {
    let mut sum: libc::c_ulong = 0;
    sum = 0 as i32 as libc::c_ulong;
    while vec != 0 {
        if vec & 1 as i32 as libc::c_ulong != 0 {
            sum ^= *mat
        }
        vec >>= 1 as i32;
        mat = mat.offset(1)
    }
    return sum;
}
/* ========================================================================= */

unsafe extern "C" fn gf2_matrix_square(
    mut square: *mut libc::c_ulong,
    mut mat: *mut libc::c_ulong,
) {
    let mut n: i32 = 0;
    n = 0 as i32;
    while n < 32 as i32 {
        *square.offset(n as isize) = gf2_matrix_times(mat, *mat.offset(n as isize));
        n += 1
    }
}
/*
     Initializes the compression dictionary from the given byte sequence
   without producing any compressed output. This function must be called
   immediately after deflateInit, deflateInit2 or deflateReset, before any
   call of deflate. The compressor and decompressor must use exactly the same
   dictionary (see inflateSetDictionary).

     The dictionary should consist of strings (byte sequences) that are likely
   to be encountered later in the data to be compressed, with the most commonly
   used strings preferably put towards the end of the dictionary. Using a
   dictionary is most useful when the data to be compressed is short and can be
   predicted with good accuracy; the data can then be compressed better than
   with the default empty dictionary.

     Depending on the size of the compression data structures selected by
   deflateInit or deflateInit2, a part of the dictionary may in effect be
   discarded, for example if the dictionary is larger than the window size in
   deflate or deflate2. Thus the strings most likely to be useful should be
   put at the end of the dictionary, not at the front. In addition, the
   current implementation of deflate will use at most the window size minus
   262 bytes of the provided dictionary.

     Upon return of this function, strm->adler is set to the adler32 value
   of the dictionary; the decompressor may later use this value to determine
   which dictionary has been used by the compressor. (The adler32 value
   applies to the whole dictionary even if only a subset of the dictionary is
   actually used by the compressor.) If a raw deflate was requested, then the
   adler32 value is not computed and strm->adler is not set.

     deflateSetDictionary returns Z_OK if success, or Z_STREAM_ERROR if a
   parameter is invalid (such as NULL dictionary) or the stream state is
   inconsistent (for example if deflate has already been called for this stream
   or if the compression method is bsort). deflateSetDictionary does not
   perform any compression: this will be done by deflate().
*/
/*
     Sets the destination stream as a complete copy of the source stream.

     This function can be useful when several compression strategies will be
   tried, for example when there are several ways of pre-processing the input
   data with a filter. The streams that will be discarded should then be freed
   by calling deflateEnd.  Note that deflateCopy duplicates the internal
   compression state which can be quite large, so this strategy is slow and
   can consume lots of memory.

     deflateCopy returns Z_OK if success, Z_MEM_ERROR if there was not
   enough memory, Z_STREAM_ERROR if the source stream state was inconsistent
   (such as zalloc being NULL). msg is left unchanged in both source and
   destination.
*/
/*
     This function is equivalent to deflateEnd followed by deflateInit,
   but does not free and reallocate all the internal compression state.
   The stream will keep the same compression level and any other attributes
   that may have been set by deflateInit2.

      deflateReset returns Z_OK if success, or Z_STREAM_ERROR if the source
   stream state was inconsistent (such as zalloc or state being NULL).
*/
/*
     Dynamically update the compression level and compression strategy.  The
   interpretation of level and strategy is as in deflateInit2.  This can be
   used to switch between compression and straight copy of the input data, or
   to switch to a different kind of input data requiring a different
   strategy. If the compression level is changed, the input available so far
   is compressed with the old level (and may be flushed); the new level will
   take effect only at the next call of deflate().

     Before the call of deflateParams, the stream state must be set as for
   a call of deflate(), since the currently available input may have to
   be compressed and flushed. In particular, strm->avail_out must be non-zero.

     deflateParams returns Z_OK if success, Z_STREAM_ERROR if the source
   stream state was inconsistent or if a parameter was invalid, Z_BUF_ERROR
   if strm->avail_out was zero.
*/
/*
    Fine tune deflate's internal compression parameters.  This should only be
  used by someone who understands the algorithm used by zlib's deflate for
  searching for the best matching string, and even then only by the most
  fanatic optimizer trying to squeeze out the last compressed bit for their
  specific input data.  Read the deflate.c source code for the meaning of the
  max_lazy, good_length, nice_length, and max_chain parameters.

    deflateTune() can be called after deflateInit() or deflateInit2(), and
  returns Z_OK on success, or Z_STREAM_ERROR for an invalid deflate stream.
*/
/*
     deflateBound() returns an upper bound on the compressed size after
   deflation of sourceLen bytes.  It must be called after deflateInit()
   or deflateInit2().  This would be used to allocate an output buffer
   for deflation in a single pass, and so would be called before deflate().
*/
/*
     deflatePrime() inserts bits in the deflate output stream.  The intent
  is that this function is used to start off the deflate output with the
  bits leftover from a previous deflate stream when appending to it.  As such,
  this function can only be used for raw deflate, and must be used before the
  first deflate() call after a deflateInit2() or deflateReset().  bits must be
  less than or equal to 16, and that many of the least significant bits of
  value will be inserted in the output.

      deflatePrime returns Z_OK if success, or Z_STREAM_ERROR if the source
   stream state was inconsistent.
*/
/*
      deflateSetHeader() provides gzip header information for when a gzip
   stream is requested by deflateInit2().  deflateSetHeader() may be called
   after deflateInit2() or deflateReset() and before the first call of
   deflate().  The text, time, os, extra field, name, and comment information
   in the provided gz_header structure are written to the gzip header (xflag is
   ignored -- the extra flags are set according to the compression level).  The
   caller must assure that, if not Z_NULL, name and comment are terminated with
   a zero byte, and that if extra is not Z_NULL, that extra_len bytes are
   available there.  If hcrc is true, a gzip header crc is included.  Note that
   the current versions of the command-line version of gzip (up through version
   1.3.x) do not support header crc's, and will report that it is a "multi-part
   gzip file" and give up.

      If deflateSetHeader is not used, the default gzip header has text false,
   the time set to zero, and os set to 255, with no extra, name, or comment
   fields.  The gzip header is returned to the default state by deflateReset().

      deflateSetHeader returns Z_OK if success, or Z_STREAM_ERROR if the source
   stream state was inconsistent.
*/
/*
ZEXTERN int ZEXPORT inflateInit2 OF((z_streamp strm,
                                     int  windowBits));

     This is another version of inflateInit with an extra parameter. The
   fields next_in, avail_in, zalloc, zfree and opaque must be initialized
   before by the caller.

     The windowBits parameter is the base two logarithm of the maximum window
   size (the size of the history buffer).  It should be in the range 8..15 for
   this version of the library. The default value is 15 if inflateInit is used
   instead. windowBits must be greater than or equal to the windowBits value
   provided to deflateInit2() while compressing, or it must be equal to 15 if
   deflateInit2() was not used. If a compressed stream with a larger window
   size is given as input, inflate() will return with the error code
   Z_DATA_ERROR instead of trying to allocate a larger window.

     windowBits can also be -8..-15 for raw inflate. In this case, -windowBits
   determines the window size. inflate() will then process raw deflate data,
   not looking for a zlib or gzip header, not generating a check value, and not
   looking for any check values for comparison at the end of the stream. This
   is for use with other formats that use the deflate compressed data format
   such as zip.  Those formats provide their own check values. If a custom
   format is developed using the raw deflate format for compressed data, it is
   recommended that a check value such as an adler32 or a crc32 be applied to
   the uncompressed data as is done in the zlib, gzip, and zip formats.  For
   most applications, the zlib format should be used as is. Note that comments
   above on the use in deflateInit2() applies to the magnitude of windowBits.

     windowBits can also be greater than 15 for optional gzip decoding. Add
   32 to windowBits to enable zlib and gzip decoding with automatic header
   detection, or add 16 to decode only the gzip format (the zlib format will
   return a Z_DATA_ERROR).  If a gzip stream is being decoded, strm->adler is
   a crc32 instead of an adler32.

     inflateInit2 returns Z_OK if success, Z_MEM_ERROR if there was not enough
   memory, Z_STREAM_ERROR if a parameter is invalid (such as a null strm). msg
   is set to null if there is no error message.  inflateInit2 does not perform
   any decompression apart from reading the zlib header if present: this will
   be done by inflate(). (So next_in and avail_in may be modified, but next_out
   and avail_out are unchanged.)
*/
/*
     Initializes the decompression dictionary from the given uncompressed byte
   sequence. This function must be called immediately after a call of inflate,
   if that call returned Z_NEED_DICT. The dictionary chosen by the compressor
   can be determined from the adler32 value returned by that call of inflate.
   The compressor and decompressor must use exactly the same dictionary (see
   deflateSetDictionary).  For raw inflate, this function can be called
   immediately after inflateInit2() or inflateReset() and before any call of
   inflate() to set the dictionary.  The application must insure that the
   dictionary that was used for compression is provided.

     inflateSetDictionary returns Z_OK if success, Z_STREAM_ERROR if a
   parameter is invalid (such as NULL dictionary) or the stream state is
   inconsistent, Z_DATA_ERROR if the given dictionary doesn't match the
   expected one (incorrect adler32 value). inflateSetDictionary does not
   perform any decompression: this will be done by subsequent calls of
   inflate().
*/
/*
    Skips invalid compressed data until a full flush point (see above the
  description of deflate with Z_FULL_FLUSH) can be found, or until all
  available input is skipped. No output is provided.

    inflateSync returns Z_OK if a full flush point has been found, Z_BUF_ERROR
  if no more input was provided, Z_DATA_ERROR if no flush point has been found,
  or Z_STREAM_ERROR if the stream structure was inconsistent. In the success
  case, the application may save the current current value of total_in which
  indicates where valid compressed data was found. In the error case, the
  application may repeatedly call inflateSync, providing more input each time,
  until success or end of the input data.
*/
/*
     Sets the destination stream as a complete copy of the source stream.

     This function can be useful when randomly accessing a large stream.  The
   first pass through the stream can periodically record the inflate state,
   allowing restarting inflate at those points when randomly accessing the
   stream.

     inflateCopy returns Z_OK if success, Z_MEM_ERROR if there was not
   enough memory, Z_STREAM_ERROR if the source stream state was inconsistent
   (such as zalloc being NULL). msg is left unchanged in both source and
   destination.
*/
/*
     This function is equivalent to inflateEnd followed by inflateInit,
   but does not free and reallocate all the internal decompression state.
   The stream will keep attributes that may have been set by inflateInit2.

      inflateReset returns Z_OK if success, or Z_STREAM_ERROR if the source
   stream state was inconsistent (such as zalloc or state being NULL).
*/
/*
     This function inserts bits in the inflate input stream.  The intent is
  that this function is used to start inflating at a bit position in the
  middle of a byte.  The provided bits will be used before any bytes are used
  from next_in.  This function should only be used with raw inflate, and
  should be used before the first inflate() call after inflateInit2() or
  inflateReset().  bits must be less than or equal to 16, and that many of the
  least significant bits of value will be inserted in the input.

      inflatePrime returns Z_OK if success, or Z_STREAM_ERROR if the source
   stream state was inconsistent.
*/
/*
      inflateGetHeader() requests that gzip header information be stored in the
   provided gz_header structure.  inflateGetHeader() may be called after
   inflateInit2() or inflateReset(), and before the first call of inflate().
   As inflate() processes the gzip stream, head->done is zero until the header
   is completed, at which time head->done is set to one.  If a zlib stream is
   being decoded, then head->done is set to -1 to indicate that there will be
   no gzip header information forthcoming.  Note that Z_BLOCK can be used to
   force inflate() to return immediately after header processing is complete
   and before any actual data is decompressed.

      The text, time, xflags, and os fields are filled in with the gzip header
   contents.  hcrc is set to true if there is a header CRC.  (The header CRC
   was valid if done is set to one.)  If extra is not Z_NULL, then extra_max
   contains the maximum number of bytes to write to extra.  Once done is true,
   extra_len contains the actual extra field length, and extra contains the
   extra field, or that field truncated if extra_max is less than extra_len.
   If name is not Z_NULL, then up to name_max characters are written there,
   terminated with a zero unless the length is greater than name_max.  If
   comment is not Z_NULL, then up to comm_max characters are written there,
   terminated with a zero unless the length is greater than comm_max.  When
   any of extra, name, or comment are not Z_NULL and the respective field is
   not present in the header, then that field is set to Z_NULL to signal its
   absence.  This allows the use of deflateSetHeader() with the returned
   structure to duplicate the header.  However if those fields are set to
   allocated memory, then the application will need to save those pointers
   elsewhere so that they can be eventually freed.

      If inflateGetHeader is not used, then the header information is simply
   discarded.  The header is always checked for validity, including the header
   CRC if present.  inflateReset() will reset the process to discard the header
   information.  The application would need to call inflateGetHeader() again to
   retrieve the header from the next gzip stream.

      inflateGetHeader returns Z_OK if success, or Z_STREAM_ERROR if the source
   stream state was inconsistent.
*/
/*
ZEXTERN int ZEXPORT inflateBackInit OF((z_streamp strm, int windowBits,
                                        unsigned char FAR *window));

     Initialize the internal stream state for decompression using inflateBack()
   calls.  The fields zalloc, zfree and opaque in strm must be initialized
   before the call.  If zalloc and zfree are Z_NULL, then the default library-
   derived memory allocation routines are used.  windowBits is the base two
   logarithm of the window size, in the range 8..15.  window is a caller
   supplied buffer of that size.  Except for special applications where it is
   assured that deflate was used with small window sizes, windowBits must be 15
   and a 32K byte window must be supplied to be able to decompress general
   deflate streams.

     See inflateBack() for the usage of these routines.

     inflateBackInit will return Z_OK on success, Z_STREAM_ERROR if any of
   the paramaters are invalid, Z_MEM_ERROR if the internal state could not
   be allocated, or Z_VERSION_ERROR if the version of the library does not
   match the version of the header file.
*/
/*
     inflateBack() does a raw inflate with a single call using a call-back
   interface for input and output.  This is more efficient than inflate() for
   file i/o applications in that it avoids copying between the output and the
   sliding window by simply making the window itself the output buffer.  This
   function trusts the application to not change the output buffer passed by
   the output function, at least until inflateBack() returns.

     inflateBackInit() must be called first to allocate the internal state
   and to initialize the state with the user-provided window buffer.
   inflateBack() may then be used multiple times to inflate a complete, raw
   deflate stream with each call.  inflateBackEnd() is then called to free
   the allocated state.

     A raw deflate stream is one with no zlib or gzip header or trailer.
   This routine would normally be used in a utility that reads zip or gzip
   files and writes out uncompressed files.  The utility would decode the
   header and process the trailer on its own, hence this routine expects
   only the raw deflate stream to decompress.  This is different from the
   normal behavior of inflate(), which expects either a zlib or gzip header and
   trailer around the deflate stream.

     inflateBack() uses two subroutines supplied by the caller that are then
   called by inflateBack() for input and output.  inflateBack() calls those
   routines until it reads a complete deflate stream and writes out all of the
   uncompressed data, or until it encounters an error.  The function's
   parameters and return types are defined above in the in_func and out_func
   typedefs.  inflateBack() will call in(in_desc, &buf) which should return the
   number of bytes of provided input, and a pointer to that input in buf.  If
   there is no input available, in() must return zero--buf is ignored in that
   case--and inflateBack() will return a buffer error.  inflateBack() will call
   out(out_desc, buf, len) to write the uncompressed data buf[0..len-1].  out()
   should return zero on success, or non-zero on failure.  If out() returns
   non-zero, inflateBack() will return with an error.  Neither in() nor out()
   are permitted to change the contents of the window provided to
   inflateBackInit(), which is also the buffer that out() uses to write from.
   The length written by out() will be at most the window size.  Any non-zero
   amount of input may be provided by in().

     For convenience, inflateBack() can be provided input on the first call by
   setting strm->next_in and strm->avail_in.  If that input is exhausted, then
   in() will be called.  Therefore strm->next_in must be initialized before
   calling inflateBack().  If strm->next_in is Z_NULL, then in() will be called
   immediately for input.  If strm->next_in is not Z_NULL, then strm->avail_in
   must also be initialized, and then if strm->avail_in is not zero, input will
   initially be taken from strm->next_in[0 .. strm->avail_in - 1].

     The in_desc and out_desc parameters of inflateBack() is passed as the
   first parameter of in() and out() respectively when they are called.  These
   descriptors can be optionally used to pass any information that the caller-
   supplied in() and out() functions need to do their job.

     On return, inflateBack() will set strm->next_in and strm->avail_in to
   pass back any unused input that was provided by the last in() call.  The
   return values of inflateBack() can be Z_STREAM_END on success, Z_BUF_ERROR
   if in() or out() returned an error, Z_DATA_ERROR if there was a format
   error in the deflate stream (in which case strm->msg is set to indicate the
   nature of the error), or Z_STREAM_ERROR if the stream was not properly
   initialized.  In the case of Z_BUF_ERROR, an input or output error can be
   distinguished using strm->next_in which will be Z_NULL only if in() returned
   an error.  If strm->next is not Z_NULL, then the Z_BUF_ERROR was due to
   out() returning non-zero.  (in() will always be called before out(), so
   strm->next_in is assured to be defined if out() returns non-zero.)  Note
   that inflateBack() cannot return Z_OK.
*/
/*
     All memory allocated by inflateBackInit() is freed.

     inflateBackEnd() returns Z_OK on success, or Z_STREAM_ERROR if the stream
   state was inconsistent.
*/
/* Return flags indicating compile-time options.

   Type sizes, two bits each, 00 = 16 bits, 01 = 32, 10 = 64, 11 = other:
    1.0: size of uInt
    3.2: size of uLong
    5.4: size of voidpf (pointer)
    7.6: size of z_off_t

   Compiler, assembler, and debug options:
    8: DEBUG
    9: ASMV or ASMINF -- use ASM code
    10: ZLIB_WINAPI -- exported functions use the WINAPI calling convention
    11: 0 (reserved)

   One-time table building (smaller code, but not thread-safe if true):
    12: BUILDFIXED -- build static block decoding tables when needed
    13: DYNAMIC_CRC_TABLE -- build CRC calculation tables when needed
    14,15: 0 (reserved)

   Library content (indicates missing functionality):
    16: NO_GZCOMPRESS -- gz* functions cannot compress (to avoid linking
                         deflate code when not needed)
    17: NO_GZIP -- deflate can't write gzip streams, and inflate can't detect
                   and decode gzip streams (to avoid linking crc code)
    18-19: 0 (reserved)

   Operation variations (changes in library functionality):
    20: PKZIP_BUG_WORKAROUND -- slightly more permissive inflate
    21: FASTEST -- deflate algorithm with only one, lowest compression level
    22,23: 0 (reserved)

   The sprintf variant used by gzprintf (zero is best):
    24: 0 = vs*, 1 = s* -- 1 means limited to 20 arguments after the format
    25: 0 = *nprintf, 1 = *printf -- 1 means gzprintf() not secure!
    26: 0 = returns value, 1 = void -- 1 means inferred string length returned

   Remainder:
    27-31: 0 (reserved)
*/
/* utility functions */
/*
     The following utility functions are implemented on top of the
   basic stream-oriented functions. To simplify the interface, some
   default options are assumed (compression level and memory usage,
   standard memory allocation functions). The source code of these
   utility functions can easily be modified if you need special options.
*/
/*
     Compresses the source buffer into the destination buffer.  sourceLen is
   the byte length of the source buffer. Upon entry, destLen is the total
   size of the destination buffer, which must be at least the value returned
   by compressBound(sourceLen). Upon exit, destLen is the actual size of the
   compressed buffer.
     This function can be used to compress a whole file at once if the
   input file is mmap'ed.
     compress returns Z_OK if success, Z_MEM_ERROR if there was not
   enough memory, Z_BUF_ERROR if there was not enough room in the output
   buffer.
*/
/*
     Compresses the source buffer into the destination buffer. The level
   parameter has the same meaning as in deflateInit.  sourceLen is the byte
   length of the source buffer. Upon entry, destLen is the total size of the
   destination buffer, which must be at least the value returned by
   compressBound(sourceLen). Upon exit, destLen is the actual size of the
   compressed buffer.

     compress2 returns Z_OK if success, Z_MEM_ERROR if there was not enough
   memory, Z_BUF_ERROR if there was not enough room in the output buffer,
   Z_STREAM_ERROR if the level parameter is invalid.
*/
/*
     compressBound() returns an upper bound on the compressed size after
   compress() or compress2() on sourceLen bytes.  It would be used before
   a compress() or compress2() call to allocate the destination buffer.
*/
/*
     Decompresses the source buffer into the destination buffer.  sourceLen is
   the byte length of the source buffer. Upon entry, destLen is the total
   size of the destination buffer, which must be large enough to hold the
   entire uncompressed data. (The size of the uncompressed data must have
   been saved previously by the compressor and transmitted to the decompressor
   by some mechanism outside the scope of this compression library.)
   Upon exit, destLen is the actual size of the compressed buffer.
     This function can be used to decompress a whole file at once if the
   input file is mmap'ed.

     uncompress returns Z_OK if success, Z_MEM_ERROR if there was not
   enough memory, Z_BUF_ERROR if there was not enough room in the output
   buffer, or Z_DATA_ERROR if the input data was corrupted or incomplete.
*/
/*
  Opens a gzip (.gz) file for reading or writing. The mode parameter
is as in fopen ("rb" or "wb") but can also include a compression level
("wb9") or a strategy: 'f' for filtered data as in "wb6f", 'h' for
Huffman only compression as in "wb1h", or 'R' for run-length encoding
as in "wb1R". (See the description of deflateInit2 for more information
about the strategy parameter.)

  gzopen can be used to read a file which is not in gzip format; in this
case gzread will directly read from the file without decompression.

  gzopen returns NULL if the file could not be opened or if there was
insufficient memory to allocate the (de)compression state; errno
can be checked to distinguish the two cases (if errno is zero, the
zlib error is Z_MEM_ERROR).  */
/*
     gzdopen() associates a gzFile with the file descriptor fd.  File
   descriptors are obtained from calls like open, dup, creat, pipe or
   fileno (in the file has been previously opened with fopen).
   The mode parameter is as in gzopen.
     The next call of gzclose on the returned gzFile will also close the
   file descriptor fd, just like fclose(fdopen(fd), mode) closes the file
   descriptor fd. If you want to keep fd open, use gzdopen(dup(fd), mode).
     gzdopen returns NULL if there was insufficient memory to allocate
   the (de)compression state.
*/
/*
     Dynamically update the compression level or strategy. See the description
   of deflateInit2 for the meaning of these parameters.
     gzsetparams returns Z_OK if success, or Z_STREAM_ERROR if the file was not
   opened for writing.
*/
/*
  Reads the given number of uncompressed bytes from the compressed file.
If the input file was not in gzip format, gzread copies the given number
of bytes into the buffer.
  gzread returns the number of uncompressed bytes actually read (0 for
end of file, -1 for error). */
/*
     Writes the given number of uncompressed bytes into the compressed file.
   gzwrite returns the number of uncompressed bytes actually written
   (0 in case of error).
*/
/*
     Converts, formats, and writes the args to the compressed file under
   control of the format string, as in fprintf. gzprintf returns the number of
   uncompressed bytes actually written (0 in case of error).  The number of
   uncompressed bytes written is limited to 4095. The caller should assure that
   this limit is not exceeded. If it is exceeded, then gzprintf() will return
   return an error (0) with nothing written. In this case, there may also be a
   buffer overflow with unpredictable consequences, which is possible only if
   zlib was compiled with the insecure functions sprintf() or vsprintf()
   because the secure snprintf() or vsnprintf() functions were not available.
*/
/*
      Writes the given null-terminated string to the compressed file, excluding
   the terminating null character.
      gzputs returns the number of characters written, or -1 in case of error.
*/
/*
      Reads bytes from the compressed file until len-1 characters are read, or
   a newline character is read and transferred to buf, or an end-of-file
   condition is encountered.  The string is then terminated with a null
   character.
      gzgets returns buf, or Z_NULL in case of error.
*/
/*
      Writes c, converted to an unsigned char, into the compressed file.
   gzputc returns the value that was written, or -1 in case of error.
*/
/*
      Reads one byte from the compressed file. gzgetc returns this byte
   or -1 in case of end of file or error.
*/
/*
      Push one character back onto the stream to be read again later.
   Only one character of push-back is allowed.  gzungetc() returns the
   character pushed, or -1 on failure.  gzungetc() will fail if a
   character has been pushed but not read yet, or if c is -1. The pushed
   character will be discarded if the stream is repositioned with gzseek()
   or gzrewind().
*/
/*
     Flushes all pending output into the compressed file. The parameter
   flush is as in the deflate() function. The return value is the zlib
   error number (see function gzerror below). gzflush returns Z_OK if
   the flush parameter is Z_FINISH and all output could be flushed.
     gzflush should be called only when strictly necessary because it can
   degrade compression.
*/
/*
      Sets the starting position for the next gzread or gzwrite on the
   given compressed file. The offset represents a number of bytes in the
   uncompressed data stream. The whence parameter is defined as in lseek(2);
   the value SEEK_END is not supported.
     If the file is opened for reading, this function is emulated but can be
   extremely slow. If the file is opened for writing, only forward seeks are
   supported; gzseek then compresses a sequence of zeroes up to the new
   starting position.

      gzseek returns the resulting offset location as measured in bytes from
   the beginning of the uncompressed stream, or -1 in case of error, in
   particular if the file is opened for writing and the new starting position
   would be before the current position.
*/
/*
     Rewinds the given file. This function is supported only for reading.

   gzrewind(file) is equivalent to (int)gzseek(file, 0L, SEEK_SET)
*/
/*
     Returns the starting position for the next gzread or gzwrite on the
   given compressed file. This position represents a number of bytes in the
   uncompressed data stream.

   gztell(file) is equivalent to gzseek(file, 0L, SEEK_CUR)
*/
/*
     Returns 1 when EOF has previously been detected reading the given
   input stream, otherwise zero.
*/
/*
     Returns 1 if file is being read directly without decompression, otherwise
   zero.
*/
/*
     Flushes all pending output if necessary, closes the compressed file
   and deallocates all the (de)compression state. The return value is the zlib
   error number (see function gzerror below).
*/
/*
     Returns the error message for the last error which occurred on the
   given compressed file. errnum is set to zlib error number. If an
   error occurred in the file system and not in the compression library,
   errnum is set to Z_ERRNO and the application may consult errno
   to get the exact error code.
*/
/*
     Clears the error and end-of-file flags for file. This is analogous to the
   clearerr() function in stdio. This is useful for continuing to read a gzip
   file that is being written concurrently.
*/
/* checksum functions */
/*
     These functions are not related to compression but are exported
   anyway because they might be useful in applications using the
   compression library.
*/
/*
     Update a running Adler-32 checksum with the bytes buf[0..len-1] and
   return the updated checksum. If buf is NULL, this function returns
   the required initial value for the checksum.
   An Adler-32 checksum is almost as reliable as a CRC32 but can be computed
   much faster. Usage example:

     uLong adler = adler32(0L, Z_NULL, 0);

     while (read_buffer(buffer, length) != EOF) {
       adler = adler32(adler, buffer, length);
     }
     if (adler != original_adler) error();
*/
/*
     Combine two Adler-32 checksums into one.  For two sequences of bytes, seq1
   and seq2 with lengths len1 and len2, Adler-32 checksums were calculated for
   each, adler1 and adler2.  adler32_combine() returns the Adler-32 checksum of
   seq1 and seq2 concatenated, requiring only adler1, adler2, and len2.
*/
/*
     Update a running CRC-32 with the bytes buf[0..len-1] and return the
   updated CRC-32. If buf is NULL, this function returns the required initial
   value for the for the crc. Pre- and post-conditioning (one's complement) is
   performed within this function so it shouldn't be done by the application.
   Usage example:

     uLong crc = crc32(0L, Z_NULL, 0);

     while (read_buffer(buffer, length) != EOF) {
       crc = crc32(crc, buffer, length);
     }
     if (crc != original_crc) error();
*/
/* ========================================================================= */
#[no_mangle]

pub unsafe extern "C" fn crc32_combine(
    mut crc1: uLong,
    mut crc2: uLong,
    mut len2: off_t,
) -> uLong {
    let mut n: i32 = 0; /* even-power-of-two zeros operator */
    let mut row: libc::c_ulong = 0; /* odd-power-of-two zeros operator */
    let mut even: [libc::c_ulong; 32] = [0; 32];
    let mut odd: [libc::c_ulong; 32] = [0; 32];
    /* degenerate case */
    if len2 == 0 as i32 as isize {
        return crc1;
    }
    /* put operator for one zero bit in odd */
    odd[0 as i32 as usize] = 0xedb88320 as isize as libc::c_ulong; /* CRC-32 polynomial */
    row = 1 as i32 as libc::c_ulong;
    n = 1 as i32;
    while n < 32 as i32 {
        odd[n as usize] = row;
        row <<= 1 as i32;
        n += 1
    }
    /* put operator for two zero bits in even */
    gf2_matrix_square(even.as_mut_ptr(), odd.as_mut_ptr());
    /* put operator for four zero bits in odd */
    gf2_matrix_square(odd.as_mut_ptr(), even.as_mut_ptr());
    loop
    /* apply len2 zeros to crc1 (first square will put the operator for one
    zero byte, eight zero bits, in even) */
      /* apply zeros operator for this bit of len2 */
    {
        gf2_matrix_square(even.as_mut_ptr(), odd.as_mut_ptr());
        if len2 & 1 as i32 as isize != 0 {
            crc1 = gf2_matrix_times(even.as_mut_ptr(), crc1)
        }
        len2 >>= 1 as i32;
        /* if no more bits set, then done */
        if len2 == 0 as i32 as isize {
            break;
        }
        /* another iteration of the loop with odd and even swapped */
        gf2_matrix_square(odd.as_mut_ptr(), even.as_mut_ptr());
        if len2 & 1 as i32 as isize != 0 {
            crc1 = gf2_matrix_times(odd.as_mut_ptr(), crc1)
        }
        len2 >>= 1 as i32;
        if !(len2 != 0 as i32 as isize) {
            break;
        }
        /* if no more bits set, then done */
    }
    /* return combined crc */
    crc1 ^= crc2;
    return crc1;
}
