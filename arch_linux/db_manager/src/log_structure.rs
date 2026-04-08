#![allow(unused,non_snake_case,non_camel_case_types)]use csv_deserializer::{create_enum, csv_dataset::CsvDataset, csv_types::CsvAny};
use std::str::FromStr;


create_enum!(name;
"amdgpu_top" => amdgputop,
"archlinux-keyring" => archlinux_keyring,
"base" => base,
"code" => code,
"coreutils" => coreutils,
"cosmic-icon-theme" => cosmic_icon_theme,
"cosmic-store" => cosmic_store,
"cpupower" => cpupower,
"ddcutil" => ddcutil,
"docker" => docker,
"eos-update" => eos_update,
"ffmpeg" => ffmpeg,
"firefox" => firefox,
"firefox-i18n-it" => firefox_iOneEightn_it,
"gc" => gc,
"geocode-glib-2" => geocode_glib_Two,
"geocode-glib-common" => geocode_glib_common,
"glances" => glances,
"glslang" => glslang,
"gnome-online-accounts" => gnome_online_accounts,
"gvfs" => gvfs,
"gvfs-afc" => gvfs_afc,
"gvfs-gphoto2" => gvfs_gphotoTwo,
"gvfs-mtp" => gvfs_mtp,
"gvfs-nfs" => gvfs_nfs,
"gvfs-smb" => gvfs_smb,
"hwdata" => hwdata,
"iana-etc" => iana_etc,
"iwd" => iwd,
"less" => less,
"lib32-libgcrypt" => libThreeTwo_libgcrypt,
"lib32-libpng" => libThreeTwo_libpng,
"lib32-libx11" => libThreeTwo_libxOneOne,
"lib32-mesa" => libThreeTwo_mesa,
"lib32-p11-kit" => libThreeTwo_pOneOne_kit,
"lib32-spirv-tools" => libThreeTwo_spirv_tools,
"lib32-systemd" => libThreeTwo_systemd,
"lib32-vulkan-icd-loader" => libThreeTwo_vulkan_icd_loader,
"lib32-vulkan-mesa-implicit-layers" => libThreeTwo_vulkan_mesa_implicit_layers,
"lib32-vulkan-radeon" => libThreeTwo_vulkan_radeon,
"libfontenc" => libfontenc,
"libgcrypt" => libgcrypt,
"libgoa" => libgoa,
"libmakepkg-dropins" => libmakepkg_dropins,
"libp11-kit" => libpOneOne_kit,
"libplacebo" => libplacebo,
"libpng" => libpng,
"libraqm" => libraqm,
"liburing" => liburing,
"libvdpau" => libvdpau,
"libx11" => libxOneOne,
"linux" => linux,
"linux-api-headers" => linux_api_headers,
"linux-headers" => linux_headers,
"linux-lts" => linux_lts,
"linux-lts-headers" => linux_lts_headers,
"linux-zen" => linux_zen,
"linux-zen-headers" => linux_zen_headers,
"mesa" => mesa,
"mutter" => mutter,
"nano" => nano,
"nfs-utils" => nfs_utils,
"nfsidmap" => nfsidmap,
"openvpn" => openvpn,
"p11-kit" => pOneOne_kit,
"resvg" => resvg,
"sbc" => sbc,
"sdl2-compat" => sdlTwo_compat,
"sdl3" => sdlThree,
"shaderc" => shaderc,
"spirv-tools" => spirv_tools,
"steam" => steam,
"systemd" => systemd,
"systemd-libs" => systemd_libs,
"systemd-resolvconf" => systemd_resolvconf,
"systemd-sysvcompat" => systemd_sysvcompat,
"tldr" => tldr,
"unrar" => unrar,
"vulkan-icd-loader" => vulkan_icd_loader,
"vulkan-mesa-implicit-layers" => vulkan_mesa_implicit_layers,
"vulkan-radeon" => vulkan_radeon,
"webkit2gtk-4.1" => webkitTwogtk_FourOne,
"webkitgtk-6.0" => webkitgtk_SixZero,
"zlib-ng" => zlib_ng,
Null,
);

create_enum!(updatetype;
"major" => major,
"minor" => minor,
"none" => none,
"patch" => patch,
"pkgrel" => pkgrel,
Null,
);

create_enum!(newversion;
"20-1" => TwoZero_One,
"2026.1-1" => TwoZeroTwoSixOne_One,
"20260203-1" => TwoZeroTwoSixZeroTwoZeroThree_One,
"20260206-1" => TwoZeroTwoSixZeroTwoZeroSix_One,
"0.404-1" => ZeroFourZeroFour_One,
"0.47.0-2" => ZeroFourSevenZero_Two,
"0.10.4-1" => ZeroOneZeroFour_One,
"0.11.2-1" => ZeroOneOneTwo_One,
"0.26.2-1" => ZeroTwoSixTwo_One,
"1.109.0-1" => OneOneZeroNineZero_One,
"1.12.0-1" => OneOneTwoZero_One,
"1.692-1" => OneSixNineTwo_One,
"1.0.0-0" => OneZeroZero_Zero,
"1.1.9-1" => OneOneNine_One,
"1.1.0.5-1" => OneOneZeroFive_One,
"1.1.4.341" => OneOneFourThreeFourOne,
"1.25.3.5-1" => OneTwoFiveThreeFive_One,
"1.29.2.1-1" => OneTwoNineTwoOne_One,
"1.4.341" => OneFourThreeFourOne,
"1.5-4" => OneFive_Four,
"1.58.1-1" => OneFiveEightOne_One,
"1.6.55-1" => OneSixFiveFive_One,
"1.7.2.4-1" => OneSevenTwoFour_One,
"1.8.13-1" => OneEightOneThree_One,
"147.0.3-1" => OneFourSevenZeroThree_One,
"147.0.3-2" => OneFourSevenZeroThree_Two,
"2.14-1" => TwoOneFour_One,
"2.2-1" => TwoTwo_One,
"2.2.5-2" => TwoTwoFive_Two,
"2.3.3-1" => TwoThreeThree_One,
"2.32.64-1" => TwoThreeTwoSixFour_One,
"2.50.5-1" => TwoFiveZeroFive_One,
"2.6.19-1" => TwoSixOneNine_One,
"2.8.5-1" => TwoEightFive_One,
"2.8.0.1-5" => TwoEightZeroOne_Five,
"259.1-1" => TwoFiveNineOne_One,
"26.1.3-1" => TwoSixOneThree_One,
"3-3" => Three_Three,
"3.11-1" => ThreeOneOne_One,
"3.26.4-5" => ThreeTwoSixFour_Five,
"3.4.4-1" => ThreeFourFour_One,
"3.4.0-3" => ThreeFourZero_Three,
"3.56.4-1" => ThreeFiveSixFour_One,
"4.5.0" => FourFiveZero,
"49.4-1" => FourNineFour_One,
"6.19-1" => SixOneNine_One,
"6.12.69-1" => SixOneTwoSixNine_One,
"6.18.8" => SixOneEightEight,
"7.351.0-5" => SevenThreeFiveOneZero_Five,
"8.2.12-1" => EightTwoOneTwo_One,
"8.7.1-1" => EightSevenOne_One,
"9.10-1" => NineOneZero_One,
Null,
);

create_enum!(oldversion;
"18-1" => OneEight_One,
"2025.5-1" => TwoZeroTwoFiveFive_One,
"20251215-1" => TwoZeroTwoFiveOneTwoOneFive_One,
"20260202-1" => TwoZeroTwoSixZeroTwoZeroTwo_One,
"0.403-1" => ZeroFourZeroThree_One,
"0.46.0-1" => ZeroFourSixZero_One,
"0.10.3-1" => ZeroOneZeroThree_One,
"0.11.0-1" => ZeroOneOneZero_One,
"0.26.1-1" => ZeroTwoSixOne_One,
"1.107.1-1" => OneOneZeroSevenOne_One,
"1.11.2-1" => OneOneOneTwo_One,
"1.691-1" => OneSixNineOne_One,
"1.0.0-0" => OneZeroZero_Zero,
"1.1.8-1" => OneOneEight_One,
"1.1.0.4-1" => OneOneZeroFour_One,
"1.1.4.335" => OneOneFourThreeThreeFive,
"1.25.3.4-1" => OneTwoFiveThreeFour_One,
"1.29.2.0-1" => OneTwoNineTwoZero_One,
"1.4.335" => OneFourThreeThreeFive,
"1.5-3" => OneFive_Three,
"1.58.0-2" => OneFiveEightZero_Two,
"1.6.54-1" => OneSixFiveFour_One,
"1.7.2.3-1" => OneSevenTwoThree_One,
"1.8.12-2" => OneEightOneTwo_Two,
"147.0.2-1" => OneFourSevenZeroTwo_One,
"2.1-1" => TwoOne_One,
"2.13-1" => TwoOneThree_One,
"2.2.3-1" => TwoTwoThree_One,
"2.3.2-1" => TwoThreeTwo_One,
"2.32.62-1" => TwoThreeTwoSixTwo_One,
"2.50.4-1" => TwoFiveZeroFour_One,
"2.6.17-1" => TwoSixOneSeven_One,
"2.8.4-1" => TwoEightFour_One,
"2.8.0.1-4" => TwoEightZeroOne_Four,
"259.0-1" => TwoFiveNineZero_One,
"259.0-2" => TwoFiveNineZero_Two,
"26.1.2-1" => TwoSixOneTwo_One,
"3-2" => Three_Two,
"3.10-1" => ThreeOneZero_One,
"3.26.4-4" => ThreeTwoSixFour_Four,
"3.4.3-3" => ThreeFourThree_Three,
"3.4.0-2" => ThreeFourZero_Two,
"3.56.3-1" => ThreeFiveSixThree_One,
"4.4.1-3" => FourFourOne_Three,
"49.3-1" => FourNineThree_One,
"6.18-1" => SixOneEight_One,
"6.18-3" => SixOneEight_Three,
"6.12.68-1" => SixOneTwoSixEight_One,
"6.18.7" => SixOneEightSeven,
"7.351.0-4" => SevenThreeFiveOneZero_Four,
"8.2.10-2" => EightTwoOneZero_Two,
"8.7.0-1" => EightSevenZero_One,
"9.9-1" => NineNine_One,
Null,
);

#[derive(Debug)]
pub enum CsvColumn{
name(Vec<name>),
updatetype(Vec<updatetype>),
newversion(Vec<newversion>),
oldversion(Vec<oldversion>),
}


pub struct CsvDataFrame{
	pub name: CsvColumn,
	pub updatetype: CsvColumn,
	pub newversion: CsvColumn,
	pub oldversion: CsvColumn,
}impl CsvDataFrame{
pub fn new(dataset: &CsvDataset) -> Self{
        let (index, _) = dataset
            .names
            .iter()
            .enumerate()
            .find(|(index, cl)| &cl.sanitized.0 == "name")
            .unwrap();
let name = CsvColumn::name(dataset.values[index].iter().map(|val| match val{
    CsvAny::Str(s) => name::from_str(s).unwrap(),
CsvAny::Null => name::Null,

    _ => panic!(),
}).collect::<Vec<name>>());
    

let (index, _) = dataset
            .names
            .iter()
            .enumerate()
            .find(|(index, cl)| &cl.sanitized.0 == "updatetype")
            .unwrap();
let updatetype = CsvColumn::updatetype(dataset.values[index].iter().map(|val| match val{
    CsvAny::Str(s) => updatetype::from_str(s).unwrap(),
CsvAny::Null => updatetype::Null,

    _ => panic!(),
}).collect::<Vec<updatetype>>());
    

let (index, _) = dataset
            .names
            .iter()
            .enumerate()
            .find(|(index, cl)| &cl.sanitized.0 == "newversion")
            .unwrap();
let newversion = CsvColumn::newversion(dataset.values[index].iter().map(|val| match val{
    CsvAny::Str(s) => newversion::from_str(s).unwrap(),
CsvAny::Null => newversion::Null,

    _ => panic!(),
}).collect::<Vec<newversion>>());
    

let (index, _) = dataset
            .names
            .iter()
            .enumerate()
            .find(|(index, cl)| &cl.sanitized.0 == "oldversion")
            .unwrap();
let oldversion = CsvColumn::oldversion(dataset.values[index].iter().map(|val| match val{
    CsvAny::Str(s) => oldversion::from_str(s).unwrap(),
CsvAny::Null => oldversion::Null,

    _ => panic!(),
}).collect::<Vec<oldversion>>());
    



        CsvDataFrame{
            name,
			updatetype,
			newversion,
			oldversion,
			
        }
    }
pub fn get_columns(&self)-> [&CsvColumn;4] {
        [&self.name,&self.updatetype,&self.newversion,&self.oldversion,]
    }}
