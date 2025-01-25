/*
 * Copyright Stalwart Labs Ltd. See the COPYING
 * file at the top-level directory of this distribution.
 *
 * Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
 * https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
 * <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
 * option. This file may not be copied, modified, or distributed
 * except according to those terms.
 */

use super::{
    multi_byte::*,
    single_byte::*,
    utf::{decoder_utf16, decoder_utf16_be, decoder_utf16_le, decoder_utf7},
    DecoderFnc,
};

pub fn charset_decoder(charset: &[u8]) -> Option<DecoderFnc> {
    let mut l_charset = [0u8; 45];

    for (dest, src) in l_charset.iter_mut().zip(charset.iter()) {
        *dest = match src {
            b'A'..=b'Z' => *src + 32,
            b'-' => b'_',
            _ => *src,
        };
    }

    hashify::map!(&l_charset[..charset.len().clamp(1, 45)],
        "850" => decoder_ibm_850,
        "866" => decoder_ibm866,
        "arabic" => decoder_iso_8859_6,
        "asmo_708" => decoder_iso_8859_6,
        "big5" => decoder_big5,
        "cp819" => decoder_iso_8859_1,
        "cp850" => decoder_ibm_850,
        "cp866" => decoder_ibm866,
        "cp936" => decoder_gbk,
        "csbig5" => decoder_big5,
        "cseuckr" => decoder_euc_kr,
        "cseucpkdfmtjapanese" => decoder_euc_jp,
        "csgb18030" => decoder_gb18030,
        "csgbk" => decoder_gbk,
        "csibm866" => decoder_ibm866,
        "csiso2022jp" => decoder_iso2022_jp,
        "csiso885913" => decoder_iso_8859_13,
        "csiso885914" => decoder_iso_8859_14,
        "csiso885915" => decoder_iso_8859_15,
        "csiso885916" => decoder_iso_8859_16,
        "csisolatin1" => decoder_iso_8859_1,
        "csisolatin2" => decoder_iso_8859_2,
        "csisolatin3" => decoder_iso_8859_3,
        "csisolatin4" => decoder_iso_8859_4,
        "csisolatin5" => decoder_iso_8859_9,
        "csisolatin6" => decoder_iso_8859_10,
        "csisolatinarabic" => decoder_iso_8859_6,
        "csisolatincyrillic" => decoder_iso_8859_5,
        "csisolatingreek" => decoder_iso_8859_7,
        "csisolatinhebrew" => decoder_iso_8859_8,
        "cskoi8r" => decoder_koi8_r,
        "cskoi8u" => decoder_koi8_u,
        "csmacintosh" => decoder_macintosh,
        "cspc850multilingual" => decoder_ibm_850,
        "csshiftjis" => decoder_shift_jis,
        "cstis620" => decoder_tis_620,
        "csutf16" => decoder_utf16,
        "csutf16be" => decoder_utf16_be,
        "csutf16le" => decoder_utf16_le,
        "csutf7" => decoder_utf7,
        "cswindows1250" => decoder_cp1250,
        "cswindows1251" => decoder_cp1251,
        "cswindows1252" => decoder_cp1252,
        "cswindows1253" => decoder_cp1253,
        "cswindows1254" => decoder_cp1254,
        "cswindows1255" => decoder_cp1255,
        "cswindows1256" => decoder_cp1256,
        "cswindows1257" => decoder_cp1257,
        "cswindows1258" => decoder_cp1258,
        "cswindows874" => decoder_windows874,
        "cyrillic" => decoder_iso_8859_5,
        "ecma_114" => decoder_iso_8859_6,
        "ecma_118" => decoder_iso_8859_7,
        "elot_928" => decoder_iso_8859_7,
        "euc_jp" => decoder_euc_jp,
        "euc_kr" => decoder_euc_kr,
        "extended_unix_code_packed_format_for_japanese" => decoder_euc_jp,
        "gb18030" => decoder_gb18030,
        "gb2312" => decoder_gb18030,
        "gbk" => decoder_gbk,
        "greek" => decoder_iso_8859_7,
        "greek8" => decoder_iso_8859_7,
        "hebrew" => decoder_iso_8859_8,
        "ibm819" => decoder_iso_8859_1,
        "ibm850" => decoder_ibm_850,
        "ibm866" => decoder_ibm866,
        "iso_2022_jp" => decoder_iso2022_jp,
        "iso_8859_1" => decoder_iso_8859_1,
        "iso_8859_10" => decoder_iso_8859_10,
        "iso_8859_10:1992" => decoder_iso_8859_10,
        "iso_8859_11" => decoder_tis_620,
        "iso_8859_13" => decoder_iso_8859_13,
        "iso_8859_14" => decoder_iso_8859_14,
        "iso_8859_14:1998" => decoder_iso_8859_14,
        "iso_8859_15" => decoder_iso_8859_15,
        "iso_8859_16" => decoder_iso_8859_16,
        "iso_8859_16:2001" => decoder_iso_8859_16,
        "iso_8859_1:1987" => decoder_iso_8859_1,
        "iso_8859_2" => decoder_iso_8859_2,
        "iso_8859_2:1987" => decoder_iso_8859_2,
        "iso_8859_3" => decoder_iso_8859_3,
        "iso_8859_3:1988" => decoder_iso_8859_3,
        "iso_8859_4" => decoder_iso_8859_4,
        "iso_8859_4:1988" => decoder_iso_8859_4,
        "iso_8859_5" => decoder_iso_8859_5,
        "iso_8859_5:1988" => decoder_iso_8859_5,
        "iso_8859_6" => decoder_iso_8859_6,
        "iso_8859_6:1987" => decoder_iso_8859_6,
        "iso_8859_7" => decoder_iso_8859_7,
        "iso_8859_7:1987" => decoder_iso_8859_7,
        "iso_8859_8" => decoder_iso_8859_8,
        "iso_8859_8:1988" => decoder_iso_8859_8,
        "iso_8859_9" => decoder_iso_8859_9,
        "iso_8859_9:1989" => decoder_iso_8859_9,
        "iso_celtic" => decoder_iso_8859_14,
        "iso_ir_100" => decoder_iso_8859_1,
        "iso_ir_101" => decoder_iso_8859_2,
        "iso_ir_109" => decoder_iso_8859_3,
        "iso_ir_110" => decoder_iso_8859_4,
        "iso_ir_126" => decoder_iso_8859_7,
        "iso_ir_127" => decoder_iso_8859_6,
        "iso_ir_138" => decoder_iso_8859_8,
        "iso_ir_144" => decoder_iso_8859_5,
        "iso_ir_148" => decoder_iso_8859_9,
        "iso_ir_157" => decoder_iso_8859_10,
        "iso_ir_199" => decoder_iso_8859_14,
        "iso_ir_226" => decoder_iso_8859_16,
        "koi8_r" => decoder_koi8_r,
        "koi8_u" => decoder_koi8_u,
        "ks_c_5601_1987" => decoder_euc_kr,
        "ks_c_5601_1989" => decoder_euc_kr,
        "l1" => decoder_iso_8859_1,
        "l10" => decoder_iso_8859_16,
        "l2" => decoder_iso_8859_2,
        "l3" => decoder_iso_8859_3,
        "l4" => decoder_iso_8859_4,
        "l5" => decoder_iso_8859_9,
        "l6" => decoder_iso_8859_10,
        "l8" => decoder_iso_8859_14,
        "latin1" => decoder_iso_8859_1,
        "latin10" => decoder_iso_8859_16,
        "latin2" => decoder_iso_8859_2,
        "latin3" => decoder_iso_8859_3,
        "latin4" => decoder_iso_8859_4,
        "latin5" => decoder_iso_8859_9,
        "latin6" => decoder_iso_8859_10,
        "latin8" => decoder_iso_8859_14,
        "latin_9" => decoder_iso_8859_15,
        "mac" => decoder_macintosh,
        "macintosh" => decoder_macintosh,
        "ms936" => decoder_gbk,
        "ms_kanji" => decoder_shift_jis,
        "shift_jis" => decoder_shift_jis,
        "tis_620" => decoder_tis_620,
        "utf_16" => decoder_utf16,
        "utf_16be" => decoder_utf16_be,
        "utf_16le" => decoder_utf16_le,
        "utf_7" => decoder_utf7,
        "windows_1250" => decoder_cp1250,
        "windows_1251" => decoder_cp1251,
        "windows_1252" => decoder_cp1252,
        "windows_1253" => decoder_cp1253,
        "windows_1254" => decoder_cp1254,
        "windows_1255" => decoder_cp1255,
        "windows_1256" => decoder_cp1256,
        "windows_1257" => decoder_cp1257,
        "windows_1258" => decoder_cp1258,
        "windows_874" => decoder_windows874,
        "windows_936" => decoder_gbk,
    )
}

#[cfg(test)]
mod tests {
    use super::charset_decoder;

    #[test]
    fn decoder_charset() {
        for input in ["gbk", "extended_unix_code_packed_format_for_japanese"] {
            if !input.is_empty() {
                assert!(
                    charset_decoder(input.as_bytes()).is_some(),
                    "Failed for {input}",
                );
            }
        }
    }
}
