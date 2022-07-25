// @generated
type DataStruct = & 'static < :: icu_normalizer :: provider :: CanonicalCompositionPassthroughV1Marker as :: icu_provider :: DataMarker > :: Yokeable ;
pub static DATA: &[(&str, DataStruct)] = &[("und", UND)];
static UND: DataStruct = &::icu_normalizer::provider::CompositionPassthroughV1 {
    first: 768u32,
    trie: ::icu_codepointtrie::CodePointTrie::from_parts(
        ::icu_codepointtrie::CodePointTrieHeader {
            high_start: 24576u32,
            shifted12_high_start: 6u16,
            index3_null_offset: 25u16,
            data_null_offset: 0u32,
            null_value: 0u32,
            trie_type: ::icu_codepointtrie::TrieType::Small,
        },
        unsafe {
            ::zerovec::ZeroVec::from_bytes_unchecked(&[
                0u8, 0u8, 64u8, 0u8, 113u8, 0u8, 175u8, 0u8, 239u8, 0u8, 47u8, 1u8, 106u8, 1u8,
                166u8, 1u8, 225u8, 1u8, 26u8, 2u8, 0u8, 0u8, 70u8, 2u8, 130u8, 2u8, 192u8, 2u8,
                255u8, 2u8, 63u8, 3u8, 127u8, 3u8, 165u8, 3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 203u8,
                3u8, 238u8, 3u8, 0u8, 0u8, 46u8, 4u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 232u8, 1u8, 7u8, 2u8, 0u8, 0u8, 16u8, 0u8, 32u8, 0u8,
                48u8, 0u8, 64u8, 0u8, 80u8, 0u8, 96u8, 0u8, 112u8, 0u8, 113u8, 0u8, 129u8, 0u8,
                145u8, 0u8, 161u8, 0u8, 175u8, 0u8, 191u8, 0u8, 207u8, 0u8, 223u8, 0u8, 239u8, 0u8,
                255u8, 0u8, 15u8, 1u8, 31u8, 1u8, 47u8, 1u8, 63u8, 1u8, 79u8, 1u8, 95u8, 1u8,
                106u8, 1u8, 122u8, 1u8, 138u8, 1u8, 154u8, 1u8, 166u8, 1u8, 182u8, 1u8, 198u8, 1u8,
                214u8, 1u8, 225u8, 1u8, 241u8, 1u8, 1u8, 2u8, 17u8, 2u8, 26u8, 2u8, 42u8, 2u8,
                58u8, 2u8, 74u8, 2u8, 0u8, 0u8, 16u8, 0u8, 32u8, 0u8, 48u8, 0u8, 70u8, 2u8, 86u8,
                2u8, 102u8, 2u8, 118u8, 2u8, 130u8, 2u8, 146u8, 2u8, 162u8, 2u8, 178u8, 2u8, 192u8,
                2u8, 208u8, 2u8, 224u8, 2u8, 240u8, 2u8, 255u8, 2u8, 15u8, 3u8, 31u8, 3u8, 47u8,
                3u8, 63u8, 3u8, 79u8, 3u8, 95u8, 3u8, 111u8, 3u8, 127u8, 3u8, 143u8, 3u8, 159u8,
                3u8, 175u8, 3u8, 165u8, 3u8, 181u8, 3u8, 197u8, 3u8, 213u8, 3u8, 0u8, 0u8, 16u8,
                0u8, 32u8, 0u8, 48u8, 0u8, 0u8, 0u8, 16u8, 0u8, 32u8, 0u8, 48u8, 0u8, 0u8, 0u8,
                16u8, 0u8, 32u8, 0u8, 48u8, 0u8, 203u8, 3u8, 219u8, 3u8, 235u8, 3u8, 251u8, 3u8,
                238u8, 3u8, 254u8, 3u8, 14u8, 4u8, 30u8, 4u8, 0u8, 0u8, 16u8, 0u8, 32u8, 0u8, 48u8,
                0u8, 46u8, 4u8, 62u8, 4u8, 78u8, 4u8, 94u8, 4u8, 0u8, 0u8, 16u8, 0u8, 32u8, 0u8,
                48u8, 0u8, 0u8, 0u8, 16u8, 0u8, 32u8, 0u8, 48u8, 0u8, 0u8, 0u8, 16u8, 0u8, 32u8,
                0u8, 48u8, 0u8, 0u8, 0u8, 16u8, 0u8, 32u8, 0u8, 48u8, 0u8, 0u8, 0u8, 16u8, 0u8,
                32u8, 0u8, 48u8, 0u8, 0u8, 0u8, 16u8, 0u8, 32u8, 0u8, 48u8, 0u8, 0u8, 0u8, 16u8,
                0u8, 32u8, 0u8, 48u8, 0u8, 0u8, 0u8, 16u8, 0u8, 32u8, 0u8, 48u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 97u8, 4u8, 113u8, 4u8, 0u8, 0u8, 0u8, 0u8, 129u8,
                4u8, 137u8, 4u8, 152u8, 4u8, 163u8, 4u8, 0u8, 0u8, 173u8, 4u8, 0u8, 0u8, 188u8,
                4u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 204u8, 4u8, 204u8, 4u8, 204u8, 4u8, 204u8, 4u8, 204u8, 4u8, 204u8, 4u8, 204u8,
                4u8, 204u8, 4u8, 204u8, 4u8, 204u8, 4u8, 204u8, 4u8, 204u8, 4u8, 204u8, 4u8, 204u8,
                4u8, 204u8, 4u8, 204u8, 4u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 204u8, 4u8, 204u8, 4u8, 219u8, 4u8,
                233u8, 4u8, 246u8, 4u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 2u8, 5u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 8u8, 5u8, 0u8, 0u8, 24u8, 5u8, 37u8, 5u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 53u8, 5u8, 61u8, 5u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 74u8, 5u8, 0u8, 0u8, 0u8, 0u8, 85u8, 5u8, 93u8, 5u8, 109u8,
                5u8, 117u8, 5u8, 195u8, 3u8, 133u8, 5u8, 148u8, 5u8, 158u8, 5u8, 189u8, 3u8, 167u8,
                5u8, 0u8, 0u8, 182u8, 5u8, 194u8, 5u8, 0u8, 0u8, 205u8, 5u8, 22u8, 4u8, 215u8, 5u8,
                226u8, 5u8, 0u8, 0u8, 195u8, 3u8, 0u8, 0u8, 236u8, 5u8, 24u8, 5u8, 246u8, 5u8,
                148u8, 2u8, 0u8, 0u8, 0u8, 0u8, 22u8, 4u8, 0u8, 0u8, 255u8, 5u8, 3u8, 3u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 8u8, 6u8, 23u8, 6u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 30u8, 6u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 184u8, 4u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 45u8,
                6u8, 61u8, 6u8, 70u8, 6u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 86u8, 6u8, 0u8, 0u8, 23u8,
                6u8, 0u8, 0u8, 0u8, 0u8, 97u8, 6u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 111u8, 6u8, 122u8,
                6u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 204u8, 4u8, 204u8, 4u8, 204u8, 4u8, 204u8, 4u8, 138u8, 6u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 66u8, 0u8, 98u8, 0u8, 130u8, 0u8, 162u8, 0u8, 166u8, 0u8, 166u8,
                0u8, 166u8, 0u8, 166u8, 0u8, 25u8, 0u8, 25u8, 0u8, 198u8, 0u8, 25u8, 0u8, 25u8,
                0u8, 222u8, 0u8, 25u8, 0u8, 254u8, 0u8, 27u8, 1u8, 59u8, 1u8, 25u8, 0u8, 25u8, 0u8,
                25u8, 0u8, 25u8, 0u8, 87u8, 1u8, 25u8, 0u8, 25u8, 0u8, 25u8, 0u8, 25u8, 0u8, 119u8,
                1u8, 25u8, 0u8, 149u8, 1u8, 181u8, 1u8, 25u8, 0u8, 25u8, 0u8, 25u8, 0u8, 25u8, 0u8,
                25u8, 0u8, 25u8, 0u8, 25u8, 0u8, 25u8, 0u8, 25u8, 0u8, 25u8, 0u8, 25u8, 0u8, 25u8,
                0u8, 25u8, 0u8, 25u8, 0u8, 25u8, 0u8, 200u8, 1u8,
            ])
        },
        unsafe {
            ::zerovec::ZeroVec::from_bytes_unchecked(&[
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 127u8, 255u8, 255u8,
                255u8, 255u8, 16u8, 64u8, 128u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 248u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 254u8, 255u8, 255u8, 255u8, 255u8, 191u8,
                182u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 255u8, 7u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                248u8, 255u8, 255u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 192u8, 159u8, 159u8, 61u8, 0u8, 0u8, 0u8, 0u8, 2u8, 0u8, 0u8, 0u8, 255u8,
                255u8, 255u8, 7u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 248u8, 15u8, 32u8, 0u8, 0u8, 192u8, 251u8, 239u8,
                62u8, 0u8, 0u8, 0u8, 0u8, 0u8, 14u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 255u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 252u8, 255u8, 255u8, 251u8, 255u8, 255u8, 255u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 16u8, 0u8, 32u8, 30u8, 255u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 80u8, 0u8, 32u8, 128u8, 176u8, 0u8, 0u8, 0u8, 64u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 72u8, 16u8, 0u8, 32u8, 0u8, 78u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 16u8, 0u8, 32u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 80u8, 0u8, 32u8, 192u8, 48u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 64u8, 0u8, 32u8, 128u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 16u8, 0u8, 32u8, 96u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 16u8, 4u8, 32u8, 96u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 88u8, 0u8, 32u8, 128u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 132u8, 0u8, 128u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 7u8, 0u8,
                15u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 7u8, 0u8,
                15u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 3u8, 0u8, 0u8, 160u8, 2u8, 8u8,
                32u8, 132u8, 16u8, 0u8, 2u8, 126u8, 61u8, 223u8, 0u8, 8u8, 32u8, 132u8, 16u8, 0u8,
                2u8, 64u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 64u8, 128u8, 6u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 32u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                254u8, 255u8, 63u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 255u8, 255u8, 255u8, 7u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 224u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 48u8, 0u8, 0u8,
                0u8, 16u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 4u8, 32u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 2u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 14u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 128u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 1u8, 0u8, 224u8, 159u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 255u8, 191u8, 255u8,
                127u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 48u8, 0u8, 16u8,
                0u8, 0u8, 0u8, 0u8, 248u8, 15u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 12u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 64u8, 0u8, 12u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 128u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                247u8, 255u8, 253u8, 33u8, 16u8, 3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 255u8,
                255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 170u8, 42u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                72u8, 0u8, 10u8, 8u8, 8u8, 8u8, 200u8, 0u8, 42u8, 3u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 255u8, 31u8, 226u8, 255u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 64u8,
                12u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 6u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 16u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 128u8, 3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 128u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 255u8, 255u8, 255u8, 255u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                252u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 6u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 128u8, 240u8, 63u8, 0u8,
                0u8, 0u8, 192u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 3u8, 0u8, 64u8,
                0u8, 0u8, 0u8, 0u8, 16u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 16u8,
                0u8, 0u8, 0u8, 255u8, 255u8, 3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 56u8, 0u8, 0u8, 0u8,
                0u8, 8u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 8u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 157u8, 193u8, 2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 64u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 32u8, 0u8, 0u8, 255u8, 255u8, 255u8,
                255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8,
                255u8, 63u8, 229u8, 127u8, 101u8, 252u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8,
                255u8, 63u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8,
                255u8, 3u8, 0u8, 0u8, 0u8, 0u8, 224u8, 0u8, 252u8, 127u8, 95u8, 219u8, 127u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 255u8, 255u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 32u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 192u8, 7u8, 0u8, 160u8, 0u8, 0u8, 0u8, 0u8, 0u8, 135u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 96u8, 0u8, 0u8, 0u8, 0u8, 240u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 24u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 192u8, 255u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 60u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 64u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 1u8, 128u8, 7u8, 0u8, 0u8, 0u8, 128u8, 0u8, 24u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 8u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 1u8, 4u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 96u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                88u8, 0u8, 32u8, 128u8, 0u8, 192u8, 31u8, 31u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 68u8, 0u8, 0u8, 64u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 1u8, 36u8, 12u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 128u8, 0u8, 128u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                192u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 8u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 1u8, 96u8, 8u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 16u8,
                0u8, 128u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 52u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 31u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                127u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 3u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 192u8, 255u8, 227u8, 7u8,
                248u8, 231u8, 15u8, 0u8, 0u8, 0u8, 60u8, 0u8, 248u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 28u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 127u8, 255u8, 255u8, 249u8,
                219u8, 7u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 64u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 240u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 127u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 240u8, 7u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                255u8, 255u8, 255u8, 63u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8,
            ])
        },
    ),
};
