use libc::{c_void, c_char, c_uchar, c_int, c_uint, c_float, uint8_t, int16_t, uint16_t, uint32_t, int64_t, uint64_t, size_t};
use avutil::*;

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
#[repr(C)]
pub enum AVCodecID {
	AV_CODEC_ID_NONE,

	/* video codecs */
	AV_CODEC_ID_MPEG1VIDEO,
	AV_CODEC_ID_MPEG2VIDEO,
	#[cfg(feature = "ff_api_xvmc")]
	AV_CODEC_ID_MPEG2VIDEO_XVMC,
	AV_CODEC_ID_H261,
	AV_CODEC_ID_H263,
	AV_CODEC_ID_RV10,
	AV_CODEC_ID_RV20,
	AV_CODEC_ID_MJPEG,
	AV_CODEC_ID_MJPEGB,
	AV_CODEC_ID_LJPEG,
	AV_CODEC_ID_SP5X,
	AV_CODEC_ID_JPEGLS,
	AV_CODEC_ID_MPEG4,
	AV_CODEC_ID_RAWVIDEO,
	AV_CODEC_ID_MSMPEG4V1,
	AV_CODEC_ID_MSMPEG4V2,
	AV_CODEC_ID_MSMPEG4V3,
	AV_CODEC_ID_WMV1,
	AV_CODEC_ID_WMV2,
	AV_CODEC_ID_H263P,
	AV_CODEC_ID_H263I,
	AV_CODEC_ID_FLV1,
	AV_CODEC_ID_SVQ1,
	AV_CODEC_ID_SVQ3,
	AV_CODEC_ID_DVVIDEO,
	AV_CODEC_ID_HUFFYUV,
	AV_CODEC_ID_CYUV,
	AV_CODEC_ID_H264,
	AV_CODEC_ID_INDEO3,
	AV_CODEC_ID_VP3,
	AV_CODEC_ID_THEORA,
	AV_CODEC_ID_ASV1,
	AV_CODEC_ID_ASV2,
	AV_CODEC_ID_FFV1,
	AV_CODEC_ID_4XM,
	AV_CODEC_ID_VCR1,
	AV_CODEC_ID_CLJR,
	AV_CODEC_ID_MDEC,
	AV_CODEC_ID_ROQ,
	AV_CODEC_ID_INTERPLAY_VIDEO,
	AV_CODEC_ID_XAN_WC3,
	AV_CODEC_ID_XAN_WC4,
	AV_CODEC_ID_RPZA,
	AV_CODEC_ID_CINEPAK,
	AV_CODEC_ID_WS_VQA,
	AV_CODEC_ID_MSRLE,
	AV_CODEC_ID_MSVIDEO1,
	AV_CODEC_ID_IDCIN,
	AV_CODEC_ID_8BPS,
	AV_CODEC_ID_SMC,
	AV_CODEC_ID_FLIC,
	AV_CODEC_ID_TRUEMOTION1,
	AV_CODEC_ID_VMDVIDEO,
	AV_CODEC_ID_MSZH,
	AV_CODEC_ID_ZLIB,
	AV_CODEC_ID_QTRLE,
	AV_CODEC_ID_TSCC,
	AV_CODEC_ID_ULTI,
	AV_CODEC_ID_QDRAW,
	AV_CODEC_ID_VIXL,
	AV_CODEC_ID_QPEG,
	AV_CODEC_ID_PNG,
	AV_CODEC_ID_PPM,
	AV_CODEC_ID_PBM,
	AV_CODEC_ID_PGM,
	AV_CODEC_ID_PGMYUV,
	AV_CODEC_ID_PAM,
	AV_CODEC_ID_FFVHUFF,
	AV_CODEC_ID_RV30,
	AV_CODEC_ID_RV40,
	AV_CODEC_ID_VC1,
	AV_CODEC_ID_WMV3,
	AV_CODEC_ID_LOCO,
	AV_CODEC_ID_WNV1,
	AV_CODEC_ID_AASC,
	AV_CODEC_ID_INDEO2,
	AV_CODEC_ID_FRAPS,
	AV_CODEC_ID_TRUEMOTION2,
	AV_CODEC_ID_BMP,
	AV_CODEC_ID_CSCD,
	AV_CODEC_ID_MMVIDEO,
	AV_CODEC_ID_ZMBV,
	AV_CODEC_ID_AVS,
	AV_CODEC_ID_SMACKVIDEO,
	AV_CODEC_ID_NUV,
	AV_CODEC_ID_KMVC,
	AV_CODEC_ID_FLASHSV,
	AV_CODEC_ID_CAVS,
	AV_CODEC_ID_JPEG2000,
	AV_CODEC_ID_VMNC,
	AV_CODEC_ID_VP5,
	AV_CODEC_ID_VP6,
	AV_CODEC_ID_VP6F,
	AV_CODEC_ID_TARGA,
	AV_CODEC_ID_DSICINVIDEO,
	AV_CODEC_ID_TIERTEXSEQVIDEO,
	AV_CODEC_ID_TIFF,
	AV_CODEC_ID_GIF,
	AV_CODEC_ID_DXA,
	AV_CODEC_ID_DNXHD,
	AV_CODEC_ID_THP,
	AV_CODEC_ID_SGI,
	AV_CODEC_ID_C93,
	AV_CODEC_ID_BETHSOFTVID,
	AV_CODEC_ID_PTX,
	AV_CODEC_ID_TXD,
	AV_CODEC_ID_VP6A,
	AV_CODEC_ID_AMV,
	AV_CODEC_ID_VB,
	AV_CODEC_ID_PCX,
	AV_CODEC_ID_SUNRAST,
	AV_CODEC_ID_INDEO4,
	AV_CODEC_ID_INDEO5,
	AV_CODEC_ID_MIMIC,
	AV_CODEC_ID_RL2,
	AV_CODEC_ID_ESCAPE124,
	AV_CODEC_ID_DIRAC,
	AV_CODEC_ID_BFI,
	AV_CODEC_ID_CMV,
	AV_CODEC_ID_MOTIONPIXELS,
	AV_CODEC_ID_TGV,
	AV_CODEC_ID_TGQ,
	AV_CODEC_ID_TQI,
	AV_CODEC_ID_AURA,
	AV_CODEC_ID_AURA2,
	AV_CODEC_ID_V210X,
	AV_CODEC_ID_TMV,
	AV_CODEC_ID_V210,
	AV_CODEC_ID_DPX,
	AV_CODEC_ID_MAD,
	AV_CODEC_ID_FRWU,
	AV_CODEC_ID_FLASHSV2,
	AV_CODEC_ID_CDGRAPHICS,
	AV_CODEC_ID_R210,
	AV_CODEC_ID_ANM,
	AV_CODEC_ID_BINKVIDEO,
	AV_CODEC_ID_IFF_ILBM,
	AV_CODEC_ID_IFF_BYTERUN1,
	AV_CODEC_ID_KGV1,
	AV_CODEC_ID_YOP,
	AV_CODEC_ID_VP8,
	AV_CODEC_ID_PICTOR,
	AV_CODEC_ID_ANSI,
	AV_CODEC_ID_A64_MULTI,
	AV_CODEC_ID_A64_MULTI5,
	AV_CODEC_ID_R10K,
	AV_CODEC_ID_MXPEG,
	AV_CODEC_ID_LAGARITH,
	AV_CODEC_ID_PRORES,
	AV_CODEC_ID_JV,
	AV_CODEC_ID_DFA,
	AV_CODEC_ID_WMV3IMAGE,
	AV_CODEC_ID_VC1IMAGE,
	AV_CODEC_ID_UTVIDEO,
	AV_CODEC_ID_BMV_VIDEO,
	AV_CODEC_ID_VBLE,
	AV_CODEC_ID_DXTORY,
	AV_CODEC_ID_V410,
	AV_CODEC_ID_XWD,
	AV_CODEC_ID_CDXL,
	AV_CODEC_ID_XBM,
	AV_CODEC_ID_ZEROCODEC,
	AV_CODEC_ID_MSS1,
	AV_CODEC_ID_MSA1,
	AV_CODEC_ID_TSCC2,
	AV_CODEC_ID_MTS2,
	AV_CODEC_ID_CLLC,
	AV_CODEC_ID_MSS2,
	AV_CODEC_ID_VP9,
	AV_CODEC_ID_AIC,
	AV_CODEC_ID_ESCAPE130_DEPRECATED,
	AV_CODEC_ID_G2M_DEPRECATED,
	AV_CODEC_ID_WEBP_DEPRECATED,
	AV_CODEC_ID_HNM4_VIDEO,
	AV_CODEC_ID_HEVC_DEPRECATED,
	AV_CODEC_ID_FIC,
	AV_CODEC_ID_ALIAS_PIX,
	AV_CODEC_ID_BRENDER_PIX_DEPRECATED,
	AV_CODEC_ID_PAF_VIDEO_DEPRECATED,
	AV_CODEC_ID_EXR_DEPRECATED,
	AV_CODEC_ID_VP7_DEPRECATED,
	AV_CODEC_ID_SANM_DEPRECATED,
	AV_CODEC_ID_SGIRLE_DEPRECATED,
	AV_CODEC_ID_MVC1_DEPRECATED,
	AV_CODEC_ID_MVC2_DEPRECATED,
	AV_CODEC_ID_HQX,
	AV_CODEC_ID_TDSC,
	AV_CODEC_ID_HQ_HQA,
	AV_CODEC_ID_HAP,
	AV_CODEC_ID_DDS,

	AV_CODEC_ID_BRENDER_PIX = MKBETAG!(b'B', b'P', b'I', b'X'),
	AV_CODEC_ID_Y41P        = MKBETAG!(b'Y', b'4', b'1', b'P'),
	AV_CODEC_ID_ESCAPE130   = MKBETAG!(b'E', b'1', b'3', b'0'),
	AV_CODEC_ID_EXR         = MKBETAG!(b'0', b'E', b'X', b'R'),
	AV_CODEC_ID_AVRP        = MKBETAG!(b'A', b'V', b'R', b'P'),

	AV_CODEC_ID_012V        = MKBETAG!(b'0', b'1', b'2', b'V'),
	AV_CODEC_ID_G2M         = MKBETAG!(  0 , b'G', b'2', b'M'),
	AV_CODEC_ID_AVUI        = MKBETAG!(b'A', b'V', b'U', b'I'),
	AV_CODEC_ID_AYUV        = MKBETAG!(b'A', b'Y', b'U', b'V'),
	AV_CODEC_ID_TARGA_Y216  = MKBETAG!(b'T', b'2', b'1', b'6'),
	AV_CODEC_ID_V308        = MKBETAG!(b'V', b'3', b'0', b'8'),
	AV_CODEC_ID_V408        = MKBETAG!(b'V', b'4', b'0', b'8'),
	AV_CODEC_ID_YUV4        = MKBETAG!(b'Y', b'U', b'V', b'4'),
	AV_CODEC_ID_SANM        = MKBETAG!(b'S', b'A', b'N', b'M'),
	AV_CODEC_ID_PAF_VIDEO   = MKBETAG!(b'P', b'A', b'F', b'V'),
	AV_CODEC_ID_AVRN        = MKBETAG!(b'A', b'V', b'R', b'n'),
	AV_CODEC_ID_CPIA        = MKBETAG!(b'C', b'P', b'I', b'A'),
	AV_CODEC_ID_XFACE       = MKBETAG!(b'X', b'F', b'A', b'C'),
	AV_CODEC_ID_SGIRLE      = MKBETAG!(b'S', b'G', b'I', b'R'),
	AV_CODEC_ID_MVC1        = MKBETAG!(b'M', b'V', b'C', b'1'),
	AV_CODEC_ID_MVC2        = MKBETAG!(b'M', b'V', b'C', b'2'),
	AV_CODEC_ID_SNOW        = MKBETAG!(b'S', b'N', b'O', b'W'),
	AV_CODEC_ID_WEBP        = MKBETAG!(b'W', b'E', b'B', b'P'),
	AV_CODEC_ID_SMVJPEG     = MKBETAG!(b'S', b'M', b'V', b'J'),
	AV_CODEC_ID_HEVC        = MKBETAG!(b'H', b'2', b'6', b'5'),
	AV_CODEC_ID_VP7         = MKBETAG!(b'V', b'P', b'7', b'0'),
	AV_CODEC_ID_APNG        = MKBETAG!(b'A', b'P', b'N', b'G'),

	/* various PCM "codecs" */
	AV_CODEC_ID_PCM_S16LE = 0x10000,
	AV_CODEC_ID_PCM_S16BE,
	AV_CODEC_ID_PCM_U16LE,
	AV_CODEC_ID_PCM_U16BE,
	AV_CODEC_ID_PCM_S8,
	AV_CODEC_ID_PCM_U8,
	AV_CODEC_ID_PCM_MULAW,
	AV_CODEC_ID_PCM_ALAW,
	AV_CODEC_ID_PCM_S32LE,
	AV_CODEC_ID_PCM_S32BE,
	AV_CODEC_ID_PCM_U32LE,
	AV_CODEC_ID_PCM_U32BE,
	AV_CODEC_ID_PCM_S24LE,
	AV_CODEC_ID_PCM_S24BE,
	AV_CODEC_ID_PCM_U24LE,
	AV_CODEC_ID_PCM_U24BE,
	AV_CODEC_ID_PCM_S24DAUD,
	AV_CODEC_ID_PCM_ZORK,
	AV_CODEC_ID_PCM_S16LE_PLANAR,
	AV_CODEC_ID_PCM_DVD,
	AV_CODEC_ID_PCM_F32BE,
	AV_CODEC_ID_PCM_F32LE,
	AV_CODEC_ID_PCM_F64BE,
	AV_CODEC_ID_PCM_F64LE,
	AV_CODEC_ID_PCM_BLURAY,
	AV_CODEC_ID_PCM_LXF,
	AV_CODEC_ID_S302M,
	AV_CODEC_ID_PCM_S8_PLANAR,
	AV_CODEC_ID_PCM_S24LE_PLANAR_DEPRECATED,
	AV_CODEC_ID_PCM_S32LE_PLANAR_DEPRECATED,
	AV_CODEC_ID_PCM_S16BE_PLANAR_DEPRECATED,
	AV_CODEC_ID_PCM_S24LE_PLANAR = MKBETAG!(  24, b'P', b'S', b'P'),
	AV_CODEC_ID_PCM_S32LE_PLANAR = MKBETAG!(  32, b'P', b'S', b'P'),
	AV_CODEC_ID_PCM_S16BE_PLANAR = MKBETAG!(b'P', b'S', b'P', 16),

	/* various ADPCM codecs */
	AV_CODEC_ID_ADPCM_IMA_QT = 0x11000,
	AV_CODEC_ID_ADPCM_IMA_WAV,
	AV_CODEC_ID_ADPCM_IMA_DK3,
	AV_CODEC_ID_ADPCM_IMA_DK4,
	AV_CODEC_ID_ADPCM_IMA_WS,
	AV_CODEC_ID_ADPCM_IMA_SMJPEG,
	AV_CODEC_ID_ADPCM_MS,
	AV_CODEC_ID_ADPCM_4XM,
	AV_CODEC_ID_ADPCM_XA,
	AV_CODEC_ID_ADPCM_ADX,
	AV_CODEC_ID_ADPCM_EA,
	AV_CODEC_ID_ADPCM_G726,
	AV_CODEC_ID_ADPCM_CT,
	AV_CODEC_ID_ADPCM_SWF,
	AV_CODEC_ID_ADPCM_YAMAHA,
	AV_CODEC_ID_ADPCM_SBPRO_4,
	AV_CODEC_ID_ADPCM_SBPRO_3,
	AV_CODEC_ID_ADPCM_SBPRO_2,
	AV_CODEC_ID_ADPCM_THP,
	AV_CODEC_ID_ADPCM_IMA_AMV,
	AV_CODEC_ID_ADPCM_EA_R1,
	AV_CODEC_ID_ADPCM_EA_R3,
	AV_CODEC_ID_ADPCM_EA_R2,
	AV_CODEC_ID_ADPCM_IMA_EA_SEAD,
	AV_CODEC_ID_ADPCM_IMA_EA_EACS,
	AV_CODEC_ID_ADPCM_EA_XAS,
	AV_CODEC_ID_ADPCM_EA_MAXIS_XA,
	AV_CODEC_ID_ADPCM_IMA_ISS,
	AV_CODEC_ID_ADPCM_G722,
	AV_CODEC_ID_ADPCM_IMA_APC,
	AV_CODEC_ID_ADPCM_VIMA_DEPRECATED,
	AV_CODEC_ID_ADPCM_VIMA    = MKBETAG!(b'V', b'I', b'M', b'A'),
	AV_CODEC_ID_ADPCM_AFC     = MKBETAG!(b'A', b'F', b'C', b' '),
	AV_CODEC_ID_ADPCM_IMA_OKI = MKBETAG!(b'O', b'K', b'I', b' '),
	AV_CODEC_ID_ADPCM_DTK     = MKBETAG!(b'D', b'T', b'K', b' '),
	AV_CODEC_ID_ADPCM_IMA_RAD = MKBETAG!(b'R', b'A', b'D', b' '),
	AV_CODEC_ID_ADPCM_G726LE  = MKBETAG!(b'6', b'2', b'7', b'G'),
	AV_CODEC_ID_ADPCM_THP_LE  = MKBETAG!(b'T', b'H', b'P', b'L'),

	/* AMR */
	AV_CODEC_ID_AMR_NB = 0x12000,
	AV_CODEC_ID_AMR_WB,

	/* RealAudio codecs*/
	AV_CODEC_ID_RA_144 = 0x13000,
	AV_CODEC_ID_RA_288,

	/* various DPCM codecs */
	AV_CODEC_ID_ROQ_DPCM = 0x14000,
	AV_CODEC_ID_INTERPLAY_DPCM,
	AV_CODEC_ID_XAN_DPCM,
	AV_CODEC_ID_SOL_DPCM,

	/* audio codecs */
	AV_CODEC_ID_MP2 = 0x15000,
	AV_CODEC_ID_MP3,
	AV_CODEC_ID_AAC,
	AV_CODEC_ID_AC3,
	AV_CODEC_ID_DTS,
	AV_CODEC_ID_VORBIS,
	AV_CODEC_ID_DVAUDIO,
	AV_CODEC_ID_WMAV1,
	AV_CODEC_ID_WMAV2,
	AV_CODEC_ID_MACE3,
	AV_CODEC_ID_MACE6,
	AV_CODEC_ID_VMDAUDIO,
	AV_CODEC_ID_FLAC,
	AV_CODEC_ID_MP3ADU,
	AV_CODEC_ID_MP3ON4,
	AV_CODEC_ID_SHORTEN,
	AV_CODEC_ID_ALAC,
	AV_CODEC_ID_WESTWOOD_SND1,
	AV_CODEC_ID_GSM,
	AV_CODEC_ID_QDM2,
	AV_CODEC_ID_COOK,
	AV_CODEC_ID_TRUESPEECH,
	AV_CODEC_ID_TTA,
	AV_CODEC_ID_SMACKAUDIO,
	AV_CODEC_ID_QCELP,
	AV_CODEC_ID_WAVPACK,
	AV_CODEC_ID_DSICINAUDIO,
	AV_CODEC_ID_IMC,
	AV_CODEC_ID_MUSEPACK7,
	AV_CODEC_ID_MLP,
	AV_CODEC_ID_GSM_MS,
	AV_CODEC_ID_ATRAC3,
	#[cfg(feature = "ff_api_voxware")]
	AV_CODEC_ID_VOXWARE,
	AV_CODEC_ID_APE,
	AV_CODEC_ID_NELLYMOSER,
	AV_CODEC_ID_MUSEPACK8,
	AV_CODEC_ID_SPEEX,
	AV_CODEC_ID_WMAVOICE,
	AV_CODEC_ID_WMAPRO,
	AV_CODEC_ID_WMALOSSLESS,
	AV_CODEC_ID_ATRAC3P,
	AV_CODEC_ID_EAC3,
	AV_CODEC_ID_SIPR,
	AV_CODEC_ID_MP1,
	AV_CODEC_ID_TWINVQ,
	AV_CODEC_ID_TRUEHD,
	AV_CODEC_ID_MP4ALS,
	AV_CODEC_ID_ATRAC1,
	AV_CODEC_ID_BINKAUDIO_RDFT,
	AV_CODEC_ID_BINKAUDIO_DCT,
	AV_CODEC_ID_AAC_LATM,
	AV_CODEC_ID_QDMC,
	AV_CODEC_ID_CELT,
	AV_CODEC_ID_G723_1,
	AV_CODEC_ID_G729,
	AV_CODEC_ID_8SVX_EXP,
	AV_CODEC_ID_8SVX_FIB,
	AV_CODEC_ID_BMV_AUDIO,
	AV_CODEC_ID_RALF,
	AV_CODEC_ID_IAC,
	AV_CODEC_ID_ILBC,
	AV_CODEC_ID_OPUS_DEPRECATED,
	AV_CODEC_ID_COMFORT_NOISE,
	AV_CODEC_ID_TAK_DEPRECATED,
	AV_CODEC_ID_METASOUND,
	AV_CODEC_ID_PAF_AUDIO_DEPRECATED,
	AV_CODEC_ID_ON2AVC,
	AV_CODEC_ID_DSS_SP,
	AV_CODEC_ID_FFWAVESYNTH     = MKBETAG!(b'F', b'F', b'W', b'S'),
	AV_CODEC_ID_SONIC           = MKBETAG!(b'S', b'O', b'N', b'C'),
	AV_CODEC_ID_SONIC_LS        = MKBETAG!(b'S', b'O', b'N', b'L'),
	AV_CODEC_ID_PAF_AUDIO       = MKBETAG!(b'P', b'A', b'F', b'A'),
	AV_CODEC_ID_OPUS            = MKBETAG!(b'O', b'P', b'U', b'S'),
	AV_CODEC_ID_TAK             = MKBETAG!(b't', b'B', b'a', b'K'),
	AV_CODEC_ID_EVRC            = MKBETAG!(b's', b'e', b'v', b'c'),
	AV_CODEC_ID_SMV             = MKBETAG!(b's', b's', b'm', b'v'),
	AV_CODEC_ID_DSD_LSBF        = MKBETAG!(b'D', b'S', b'D', b'L'),
	AV_CODEC_ID_DSD_MSBF        = MKBETAG!(b'D', b'S', b'D', b'M'),
	AV_CODEC_ID_DSD_LSBF_PLANAR = MKBETAG!(b'D', b'S', b'D', b'1'),
	AV_CODEC_ID_DSD_MSBF_PLANAR = MKBETAG!(b'D', b'S', b'D', b'8'),
	AV_CODEC_ID_4GV             = MKBETAG!(b's', b'4', b'g', b'v'),

	/* subtitle codecs */
	AV_CODEC_ID_DVD_SUBTITLE   = 0x17000,
	AV_CODEC_ID_DVB_SUBTITLE,
	AV_CODEC_ID_TEXT,
	AV_CODEC_ID_XSUB,
	AV_CODEC_ID_SSA,
	AV_CODEC_ID_MOV_TEXT,
	AV_CODEC_ID_HDMV_PGS_SUBTITLE,
	AV_CODEC_ID_DVB_TELETEXT,
	AV_CODEC_ID_SRT,
	AV_CODEC_ID_MICRODVD           = MKBETAG!(b'm', b'D', b'V', b'D'),
	AV_CODEC_ID_EIA_608            = MKBETAG!(b'c', b'6', b'0', b'8'),
	AV_CODEC_ID_JACOSUB            = MKBETAG!(b'J', b'S', b'U', b'B'),
	AV_CODEC_ID_SAMI               = MKBETAG!(b'S', b'A', b'M', b'I'),
	AV_CODEC_ID_REALTEXT           = MKBETAG!(b'R', b'T', b'X', b'T'),
	AV_CODEC_ID_STL                = MKBETAG!(b'S', b'p', b'T', b'L'),
	AV_CODEC_ID_SUBVIEWER1         = MKBETAG!(b'S', b'b', b'V', b'1'),
	AV_CODEC_ID_SUBVIEWER          = MKBETAG!(b'S', b'u', b'b', b'V'),
	AV_CODEC_ID_SUBRIP             = MKBETAG!(b'S', b'R', b'i', b'p'),
	AV_CODEC_ID_WEBVTT             = MKBETAG!(b'W', b'V', b'T', b'T'),
	AV_CODEC_ID_MPL2               = MKBETAG!(b'M', b'P', b'L', b'2'),
	AV_CODEC_ID_VPLAYER            = MKBETAG!(b'V', b'P', b'l', b'r'),
	AV_CODEC_ID_PJS                = MKBETAG!(b'P', b'h', b'J', b'S'),
	AV_CODEC_ID_ASS                = MKBETAG!(b'A', b'S', b'S', b' '),
	AV_CODEC_ID_HDMV_TEXT_SUBTITLE = MKBETAG!(b'B', b'D', b'T', b'X'),

	/* other specific kind of codecs (generally used for attachments) */
	AV_CODEC_ID_TTF           = 0x18000,
	AV_CODEC_ID_BINTEXT       = MKBETAG!(b'B', b'T', b'X', b'T'),
	AV_CODEC_ID_XBIN          = MKBETAG!(b'X', b'B', b'I', b'N'),
	AV_CODEC_ID_IDF           = MKBETAG!(  0 , b'I', b'D', b'F'),
	AV_CODEC_ID_OTF           = MKBETAG!(  0 , b'O', b'T', b'F'),
	AV_CODEC_ID_SMPTE_KLV     = MKBETAG!(b'K', b'L', b'V', b'A'),
	AV_CODEC_ID_DVD_NAV       = MKBETAG!(b'D', b'N', b'A', b'V'),
	AV_CODEC_ID_TIMED_ID3     = MKBETAG!(b'T', b'I', b'D', b'3'),
	AV_CODEC_ID_BIN_DATA      = MKBETAG!(b'D', b'A', b'T', b'A'),

	AV_CODEC_ID_PROBE = 0x19000,

	AV_CODEC_ID_MPEG2TS      = 0x20000,
	AV_CODEC_ID_MPEG4SYSTEMS = 0x20001,
	AV_CODEC_ID_FFMETADATA   = 0x21000,
}

pub const AV_CODEC_ID_VIMA: AVCodecID = AVCodecID::AV_CODEC_ID_ADPCM_VIMA;

pub const AV_CODEC_ID_FIRST_SUBTITLE: AVCodecID = AVCodecID::AV_CODEC_ID_DVD_SUBTITLE;
pub const AV_CODEC_ID_FIRST_AUDIO:    AVCodecID = AVCodecID::AV_CODEC_ID_PCM_S16LE;
pub const AV_CODEC_ID_FIRST_UNKNOWN:  AVCodecID = AVCodecID::AV_CODEC_ID_TTF;

#[derive(Debug)]
#[repr(C)]
pub struct AVCodecDescriptor {
	id:   AVCodecID,
	kind: AVMediaType,

	name:      *const c_char,
	long_name: *const c_char,

	props: c_int,
	mime_types: *const *const c_char,
}

pub const AV_CODEC_PROP_INTRA_ONLY: c_int = 1 << 0;
pub const AV_CODEC_PROP_LOSSY:      c_int = 1 << 1;
pub const AV_CODEC_PROP_LOSSLESS:   c_int = 1 << 2;
pub const AV_CODEC_PROP_REORDER:    c_int = 1 << 3;
pub const AV_CODEC_PROP_BITMAP_SUB: c_int = 1 << 16;
pub const AV_CODEC_PROP_TEXT_SUB:   c_int = 1 << 17;

pub const AV_INPUT_BUFFER_PADDING_SIZE: c_int = 32;
pub const AV_MIN_BUFFER_SIZE:           c_int = 16384;

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
#[repr(C)]
pub enum Motion_Est_ID {
	ME_ZERO = 1,
	ME_FULL,
	ME_LOG,
	ME_PHODS,
	ME_EPZS,
	ME_X1,
	ME_HEX,
	ME_UMH,
	ME_TESA,
	ME_ITER = 50,
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
#[repr(C)]
pub enum AVDiscard{
	AVDISCARD_NONE     = -16,
	AVDISCARD_DEFAULT  = 0,
	AVDISCARD_NONREF   = 8,
	AVDISCARD_BIDIR    = 16,
	AVDISCARD_NONINTRA = 24,
	AVDISCARD_NONKEY   = 32,
	AVDISCARD_ALL      = 48,
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
#[repr(C)]
pub enum AVAudioServiceType {
	AV_AUDIO_SERVICE_TYPE_MAIN              = 0,
	AV_AUDIO_SERVICE_TYPE_EFFECTS           = 1,
	AV_AUDIO_SERVICE_TYPE_VISUALLY_IMPAIRED = 2,
	AV_AUDIO_SERVICE_TYPE_HEARING_IMPAIRED  = 3,
	AV_AUDIO_SERVICE_TYPE_DIALOGUE          = 4,
	AV_AUDIO_SERVICE_TYPE_COMMENTARY        = 5,
	AV_AUDIO_SERVICE_TYPE_EMERGENCY         = 6,
	AV_AUDIO_SERVICE_TYPE_VOICE_OVER        = 7,
	AV_AUDIO_SERVICE_TYPE_KARAOKE           = 8,
	AV_AUDIO_SERVICE_TYPE_NB,
}

#[derive(Debug)]
#[repr(C)]
pub struct RcOverride {
	start_frame: c_int,
	end_frame: c_int,
	qscale: c_int,
	quality_factor: c_float,
}

pub const AV_CODEC_FLAG_UNALIGNED:       c_uint = 0x0001;
pub const AV_CODEC_FLAG_QSCALE:          c_uint = 0x0002;
pub const AV_CODEC_FLAG_4MV:             c_uint = 0x0004;
pub const AV_CODEC_FLAG_OUTPUT_CORRUPT:  c_uint = 0x0008;
pub const AV_CODEC_FLAG_QPEL:            c_uint = 0x0010;
pub const AV_CODEC_FLAG_GMC:             c_uint = 0x0020;
pub const AV_CODEC_FLAG_MV0:             c_uint = 0x0040;
pub const AV_CODEC_FLAG_INPUT_PRESERVED: c_uint = 0x0100;
pub const AV_CODEC_FLAG_PASS1:           c_uint = 0x0200;
pub const AV_CODEC_FLAG_PASS2:           c_uint = 0x0400;
pub const AV_CODEC_FLAG_GRAY:            c_uint = 0x2000;
pub const AV_CODEC_FLAG_EMU_EDGE:        c_uint = 0x4000;
pub const AV_CODEC_FLAG_PSNR:            c_uint = 0x8000;
pub const AV_CODEC_FLAG_TRUNCATED:       c_uint = 0x00010000;
pub const AV_CODEC_FLAG_NORMALIZE_AQP:   c_uint = 0x00020000;
pub const AV_CODEC_FLAG_INTERLACED_DCT:  c_uint = 0x00040000;
pub const AV_CODEC_FLAG_LOW_DELAY:       c_uint = 0x00080000;
pub const AV_CODEC_FLAG_GLOBAL_HEADER:   c_uint = 0x00400000;
pub const AV_CODEC_FLAG_BITEXACT:        c_uint = 0x00800000;
pub const AV_CODEC_FLAG_AC_PRED:         c_uint = 0x01000000;
pub const AV_CODEC_FLAG_LOOP_FILTER:     c_uint = 0x00000800;
pub const AV_CODEC_FLAG_INTERLACED_ME:   c_uint = 0x20000000;
pub const AV_CODEC_FLAG_CLOSED_GOP:      c_uint = 0x80000000;

pub const AV_CODEC_FLAG2_FAST:                c_uint = 0x00000001;
pub const AV_CODEC_FLAG2_NO_OUTPUT:           c_uint = 0x00000004;
pub const AV_CODEC_FLAG2_LOCAL_HEADER:        c_uint = 0x00000008;
pub const AV_CODEC_FLAG2_DROP_FRAME_TIMECODE: c_uint = 0x00002000;
pub const AV_CODEC_FLAG2_IGNORE_CROP:         c_uint = 0x00010000;
pub const AV_CODEC_FLAG2_CHUNKS:              c_uint = 0x00008000;
pub const AV_CODEC_FLAG2_SHOW_ALL:            c_uint = 0x00400000;
pub const AV_CODEC_FLAG2_EXPORT_MVS:          c_uint = 0x10000000;
pub const AV_CODEC_FLAG2_SKIP_MANUAL:         c_uint = 0x20000000;

pub const AV_CODEC_CAP_DRAW_HORIZ_BAND:     c_uint = 0x0001;
pub const AV_CODEC_CAP_DR1:                 c_uint = 0x0002;
pub const AV_CODEC_CAP_TRUNCATED:           c_uint = 0x0008;
pub const AV_CODEC_CAP_HWACCEL:             c_uint = 0x0010;
pub const AV_CODEC_CAP_DELAY:               c_uint = 0x0020;
pub const AV_CODEC_CAP_SMALL_LAST_FRAME:    c_uint = 0x0040;
pub const AV_CODEC_CAP_HWACCEL_VDPAU:       c_uint = 0x0080;
pub const AV_CODEC_CAP_SUBFRAMES:           c_uint = 0x0100;
pub const AV_CODEC_CAP_EXPERIMENTAL:        c_uint = 0x0200;
pub const AV_CODEC_CAP_CHANNEL_CONF:        c_uint = 0x0400;
pub const AV_CODEC_CAP_NEG_LINESIZES:       c_uint = 0x0800;
pub const AV_CODEC_CAP_FRAME_THREADS:       c_uint = 0x1000;
pub const AV_CODEC_CAP_SLICE_THREADS:       c_uint = 0x2000;
pub const AV_CODEC_CAP_PARAM_CHANGE:        c_uint = 0x4000;
pub const AV_CODEC_CAP_AUTO_THREADS:        c_uint = 0x8000;
pub const AV_CODEC_CAP_VARIABLE_FRAME_SIZE: c_uint = 0x10000;
pub const AV_CODEC_CAP_INTRA_ONLY:          c_uint = 0x40000000;
pub const AV_CODEC_CAP_LOSSLESS:            c_uint = 0x80000000;

pub const HWACCEL_CODEC_CAP_EXPERIMENTAL: c_uint = 0x0200;

pub const MB_TYPE_INTRA4x4:   c_int = 0x0001;
pub const MB_TYPE_INTRA16x16: c_int = 0x0002;
pub const MB_TYPE_INTRA_PCM:  c_int = 0x0004;
pub const MB_TYPE_16x16:      c_int = 0x0008;
pub const MB_TYPE_16x8:       c_int = 0x0010;
pub const MB_TYPE_8x16:       c_int = 0x0020;
pub const MB_TYPE_8x8:        c_int = 0x0040;
pub const MB_TYPE_INTERLACED: c_int = 0x0080;
pub const MB_TYPE_DIRECT2:    c_int = 0x0100;
pub const MB_TYPE_ACPRED:     c_int = 0x0200;
pub const MB_TYPE_GMC:        c_int = 0x0400;
pub const MB_TYPE_SKIP:       c_int = 0x0800;
pub const MB_TYPE_P0L0:       c_int = 0x1000;
pub const MB_TYPE_P1L0:       c_int = 0x2000;
pub const MB_TYPE_P0L1:       c_int = 0x4000;
pub const MB_TYPE_P1L1:       c_int = 0x8000;
pub const MB_TYPE_L0:         c_int = MB_TYPE_P0L0 | MB_TYPE_P1L0;
pub const MB_TYPE_L1:         c_int = MB_TYPE_P0L1 | MB_TYPE_P1L1;
pub const MB_TYPE_L0L1:       c_int = MB_TYPE_L0   | MB_TYPE_L1;
pub const MB_TYPE_QUANT:      c_int = 0x00010000;
pub const MB_TYPE_CBP:        c_int = 0x00020000;

#[derive(Debug)]
#[repr(C)]
pub struct AVPanScan {
	id: c_int,
	width: c_int,
	height: c_int,
	position: [[int16_t; 2]; 3],
}

pub const FF_QSCALE_TYPE_MPEG1: c_int = 0;
pub const FF_QSCALE_TYPE_MPEG2: c_int = 1;
pub const FF_QSCALE_TYPE_H264:  c_int = 2;
pub const FF_QSCALE_TYPE_VP56:  c_int = 3;

pub const FF_BUFFER_TYPE_INTERNAL: c_int = 1;
pub const FF_BUFFER_TYPE_USER:     c_int = 2;
pub const FF_BUFFER_TYPE_SHARED:   c_int = 4;
pub const FF_BUFFER_TYPE_COPY:     c_int = 8;

pub const FF_BUFFER_HINTS_VALID:    c_int = 0x01;
pub const FF_BUFFER_HINTS_READABLE: c_int = 0x02;
pub const FF_BUFFER_HINTS_PRESERVE: c_int = 0x04;
pub const FF_BUFFER_HINTS_REUSABLE: c_int = 0x08;

pub const AV_GET_BUFFER_FLAG_REF: c_int = 1 << 0;

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
#[repr(C)]
pub enum AVPacketSideDataType {
	AV_PKT_DATA_PALETTE,
	AV_PKT_DATA_NEW_EXTRADATA,
	AV_PKT_DATA_PARAM_CHANGE,
	AV_PKT_DATA_H263_MB_INFO,
	AV_PKT_DATA_REPLAYGAIN,
	AV_PKT_DATA_DISPLAYMATRIX,
	AV_PKT_DATA_STEREO3D,
	AV_PKT_DATA_AUDIO_SERVICE_TYPE,
	AV_PKT_DATA_QUALITY_STATS,
	AV_PKT_DATA_SKIP_SAMPLES = 70,
	AV_PKT_DATA_JP_DUALMONO,
	AV_PKT_DATA_STRINGS_METADATA,
	AV_PKT_DATA_SUBTITLE_POSITION,
	AV_PKT_DATA_MATROSKA_BLOCKADDITIONAL,
	AV_PKT_DATA_WEBVTT_IDENTIFIER,
	AV_PKT_DATA_WEBVTT_SETTINGS,
	AV_PKT_DATA_METADATA_UPDATE,
}

#[derive(Debug)]
#[repr(C)]
pub struct AVPacketSideData {
	pub data: *mut uint8_t,
	pub size: c_int,
	pub kind: AVPacketSideDataType,
}

#[repr(C)]
pub struct AVPacket {
	pub buf: *mut AVBufferRef,
	pub pts: int64_t,
	pub dts: int64_t,
	pub data: *mut uint8_t,
	pub size: c_int,
	pub stream_index: c_int,
	pub flags: c_int,
	pub side_data: *mut AVPacketSideData,
	pub side_data_elems: c_int,
	pub duration: c_int,
	#[cfg(feature = "ff_api_destruct_packet")]
	pub destruct: Option<extern fn(*mut AVPacket)>,
	#[cfg(feature = "ff_api_destruct_packet")]
	pub private: *mut c_void,
	pub pos: int64_t,
	pub convergence_duration: int64_t,
}

pub const AV_PKT_FLAG_KEY:     c_int = 0x0001;
pub const AV_PKT_FLAG_CORRUPT: c_int = 0x0002;

pub const AV_SIDE_DATA_PARAM_CHANGE_CHANNEL_COUNT:  c_int = 0x0001;
pub const AV_SIDE_DATA_PARAM_CHANGE_CHANNEL_LAYOUT: c_int = 0x0002;
pub const AV_SIDE_DATA_PARAM_CHANGE_SAMPLE_RATE:    c_int = 0x0004;
pub const AV_SIDE_DATA_PARAM_CHANGE_DIMENSIONS:     c_int = 0x0008;

pub type AVCodecInternal = c_void;

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
#[repr(C)]
pub enum AVFieldOrder {
	AV_FIELD_UNKNOWN,
	AV_FIELD_PROGRESSIVE,
	AV_FIELD_TT,
	AV_FIELD_BB,
	AV_FIELD_TB,
	AV_FIELD_BT,
}

pub const FF_COMPRESSION_DEFAULT: c_int = -1;
pub const FF_ASPECT_EXTENDED:     c_int = 15;
pub const FF_RC_STRATEGY_XVID:    c_int = 1;
pub const FF_DEFAULT_QUANT_BIAS:  c_int = 999999;

pub const FF_PRED_LEFT:   c_int = 0;
pub const FF_PRED_PLANE:  c_int = 1;
pub const FF_PRED_MEDIAN: c_int = 2;

pub const FF_CMP_SAD:    c_int = 0;
pub const FF_CMP_SSE:    c_int = 1;
pub const FF_CMP_SATD:   c_int = 2;
pub const FF_CMP_DCT:    c_int = 3;
pub const FF_CMP_PSNR:   c_int = 4;
pub const FF_CMP_BIT:    c_int = 5;
pub const FF_CMP_RD:     c_int = 6;
pub const FF_CMP_ZERO:   c_int = 7;
pub const FF_CMP_VSAD:   c_int = 8;
pub const FF_CMP_VSSE:   c_int = 9;
pub const FF_CMP_NSSE:   c_int = 10;
pub const FF_CMP_W53:    c_int = 11;
pub const FF_CMP_W97:    c_int = 12;
pub const FF_CMP_DCTMAX: c_int = 13;
pub const FF_CMP_DCT264: c_int = 14;
pub const FF_CMP_CHROMA: c_int = 256;

pub const SLICE_FLAG_CODED_ORDER: c_int = 0x0001;
pub const SLICE_FLAG_ALLOW_FIELD: c_int = 0x0002;
pub const SLICE_FLAG_ALLOW_PLANE: c_int = 0x0004;

pub const FF_MB_DECISION_SIMPLE: c_int = 0;
pub const FF_MB_DECISION_BITS:   c_int = 1;
pub const FF_MB_DECISION_RD:     c_int = 2;

pub const FF_CODER_TYPE_VLC:     c_int = 0;
pub const FF_CODER_TYPE_AC:      c_int = 1;
pub const FF_CODER_TYPE_RAW:     c_int = 2;
pub const FF_CODER_TYPE_RLE:     c_int = 3;
pub const FF_CODER_TYPE_DEFLATE: c_int = 4;

pub const FF_BUG_AUTODETECT:       c_int = 1;
pub const FF_BUG_OLD_MSMPEG4:      c_int = 2;
pub const FF_BUG_XVID_ILACE:       c_int = 4;
pub const FF_BUG_UMP4:             c_int = 8;
pub const FF_BUG_NO_PADDING:       c_int = 16;
pub const FF_BUG_AMV:              c_int = 32;
pub const FF_BUG_AC_VLC:           c_int = 0;
pub const FF_BUG_QPEL_CHROMA:      c_int = 64;
pub const FF_BUG_STD_QPEL:         c_int = 128;
pub const FF_BUG_QPEL_CHROMA2:     c_int = 256;
pub const FF_BUG_DIRECT_BLOCKSIZE: c_int = 512;
pub const FF_BUG_EDGE:             c_int = 1024;
pub const FF_BUG_HPEL_CHROMA:      c_int = 2048;
pub const FF_BUG_DC_CLIP:          c_int = 4096;
pub const FF_BUG_MS:               c_int = 8192;
pub const FF_BUG_TRUNCATED:        c_int = 16384;

pub const FF_COMPLIANCE_VERY_STRICT:  c_int = 2;
pub const FF_COMPLIANCE_STRICT:       c_int = 1;
pub const FF_COMPLIANCE_NORMAL:       c_int = 0;
pub const FF_COMPLIANCE_UNOFFICIAL:   c_int = -1;
pub const FF_COMPLIANCE_EXPERIMENTAL: c_int = -2;

pub const FF_EC_GUESS_MVS:   c_int = 1;
pub const FF_EC_DEBLOCK:     c_int = 2;
pub const FF_EC_FAVOR_INTER: c_int = 256;

pub const FF_DEBUG_PICT_INFO:   c_int = 1;
pub const FF_DEBUG_RC:          c_int = 2;
pub const FF_DEBUG_BITSTREAM:   c_int = 4;
pub const FF_DEBUG_MB_TYPE:     c_int = 8;
pub const FF_DEBUG_QP:          c_int = 16;
pub const FF_DEBUG_MV:          c_int = 32;
pub const FF_DEBUG_DCT_COEFF:   c_int = 0x00000040;
pub const FF_DEBUG_SKIP:        c_int = 0x00000080;
pub const FF_DEBUG_STARTCODE:   c_int = 0x00000100;
pub const FF_DEBUG_PTS:         c_int = 0x00000200;
pub const FF_DEBUG_ER:          c_int = 0x00000400;
pub const FF_DEBUG_MMCO:        c_int = 0x00000800;
pub const FF_DEBUG_BUGS:        c_int = 0x00001000;
pub const FF_DEBUG_VIS_QP:      c_int = 0x00002000;
pub const FF_DEBUG_VIS_MB_TYPE: c_int = 0x00004000;
pub const FF_DEBUG_BUFFERS:     c_int = 0x00008000;
pub const FF_DEBUG_THREADS:     c_int = 0x00010000;
pub const FF_DEBUG_GREEN_MD:    c_int = 0x00800000;
pub const FF_DEBUG_NOMC:        c_int = 0x01000000;

pub const AV_EF_CRCCHECK:   c_int = 1 << 0;
pub const AV_EF_BITSTREAM:  c_int = 1 << 1;
pub const AV_EF_BUFFER:     c_int = 1 << 2;
pub const AV_EF_EXPLODE:    c_int = 1 << 3;
pub const AV_EF_IGNORE_ERR: c_int = 1 << 15;
pub const AV_EF_CAREFUL:    c_int = 1 << 16;
pub const AV_EF_COMPLIANT:  c_int = 1 << 17;
pub const AV_EF_AGGRESSIVE: c_int = 1 << 18;

pub const FF_DCT_AUTO:    c_int = 0;
pub const FF_DCT_FASTINT: c_int = 1;
pub const FF_DCT_INT:     c_int = 2;
pub const FF_DCT_MMX:     c_int = 3;
pub const FF_DCT_ALTIVEC: c_int = 5;
pub const FF_DCT_FAAN:    c_int = 6;

pub const FF_IDCT_AUTO:          c_int = 0;
pub const FF_IDCT_INT:           c_int = 1;
pub const FF_IDCT_SIMPLE:        c_int = 2;
pub const FF_IDCT_SIMPLEMMX:     c_int = 3;
pub const FF_IDCT_ARM:           c_int = 7;
pub const FF_IDCT_ALTIVEC:       c_int = 8;
pub const FF_IDCT_SH4:           c_int = 9;
pub const FF_IDCT_SIMPLEARM:     c_int = 10;
pub const FF_IDCT_IPP:           c_int = 13;
pub const FF_IDCT_XVID:          c_int = 14;
pub const FF_IDCT_XVIDMMX:       c_int = 14;
pub const FF_IDCT_SIMPLEARMV5TE: c_int = 16;
pub const FF_IDCT_SIMPLEARMV6:   c_int = 17;
pub const FF_IDCT_SIMPLEVIS:     c_int = 18;
pub const FF_IDCT_FAAN:          c_int = 20;
pub const FF_IDCT_SIMPLENEON:    c_int = 22;
pub const FF_IDCT_SIMPLEALPHA:   c_int = 23;
pub const FF_IDCT_SIMPLEAUTO:    c_int = 128;

pub const FF_THREAD_FRAME: c_int = 1;
pub const FF_THREAD_SLICE: c_int = 2;

pub const FF_PROFILE_UNKNOWN:  c_int = -99;
pub const FF_PROFILE_RESERVED: c_int = -100;

pub const FF_PROFILE_AAC_MAIN:      c_int = 0;
pub const FF_PROFILE_AAC_LOW:       c_int = 1;
pub const FF_PROFILE_AAC_SSR:       c_int = 2;
pub const FF_PROFILE_AAC_LTP:       c_int = 3;
pub const FF_PROFILE_AAC_HE:        c_int = 4;
pub const FF_PROFILE_AAC_HE_V2:     c_int = 28;
pub const FF_PROFILE_AAC_LD:        c_int = 22;
pub const FF_PROFILE_AAC_ELD:       c_int = 38;
pub const FF_PROFILE_MPEG2_AAC_LOW: c_int = 128;
pub const FF_PROFILE_MPEG2_AAC_HE:  c_int = 131;

pub const FF_PROFILE_DTS:         c_int = 20;
pub const FF_PROFILE_DTS_ES:      c_int = 30;
pub const FF_PROFILE_DTS_96_24:   c_int = 40;
pub const FF_PROFILE_DTS_HD_HRA:  c_int = 50;
pub const FF_PROFILE_DTS_HD_MA:   c_int = 60;
pub const FF_PROFILE_DTS_EXPRESS: c_int = 70;

pub const FF_PROFILE_MPEG2_422:          c_int = 0;
pub const FF_PROFILE_MPEG2_HIGH:         c_int = 1;
pub const FF_PROFILE_MPEG2_SS:           c_int = 2;
pub const FF_PROFILE_MPEG2_SNR_SCALABLE: c_int = 3;
pub const FF_PROFILE_MPEG2_MAIN:         c_int = 4;
pub const FF_PROFILE_MPEG2_SIMPLE:       c_int = 5;

pub const FF_PROFILE_H264_CONSTRAINED: c_int = 1 << 9;
pub const FF_PROFILE_H264_INTRA:       c_int = 1 << 11;

pub const FF_PROFILE_H264_BASELINE:             c_int = 66;
pub const FF_PROFILE_H264_CONSTRAINED_BASELINE: c_int = 66 | FF_PROFILE_H264_CONSTRAINED;
pub const FF_PROFILE_H264_MAIN:                 c_int = 77;
pub const FF_PROFILE_H264_EXTENDED:             c_int = 88;
pub const FF_PROFILE_H264_HIGH:                 c_int = 100;
pub const FF_PROFILE_H264_HIGH_10:              c_int = 110;
pub const FF_PROFILE_H264_HIGH_10_INTRA:        c_int = 110 | FF_PROFILE_H264_INTRA;
pub const FF_PROFILE_H264_HIGH_422:             c_int = 122;
pub const FF_PROFILE_H264_HIGH_422_INTRA:       c_int = 122 | FF_PROFILE_H264_INTRA;
pub const FF_PROFILE_H264_HIGH_444:             c_int = 144;
pub const FF_PROFILE_H264_HIGH_444_PREDICTIVE:  c_int = 244;
pub const FF_PROFILE_H264_HIGH_444_INTRA:       c_int = 244 | FF_PROFILE_H264_INTRA;
pub const FF_PROFILE_H264_CAVLC_444:            c_int = 44;

pub const FF_PROFILE_VC1_SIMPLE:   c_int = 0;
pub const FF_PROFILE_VC1_MAIN:     c_int = 1;
pub const FF_PROFILE_VC1_COMPLEX:  c_int = 2;
pub const FF_PROFILE_VC1_ADVANCED: c_int = 3;

pub const FF_PROFILE_MPEG4_SIMPLE:                    c_int = 0;
pub const FF_PROFILE_MPEG4_SIMPLE_SCALABLE:           c_int = 1;
pub const FF_PROFILE_MPEG4_CORE:                      c_int = 2;
pub const FF_PROFILE_MPEG4_MAIN:                      c_int = 3;
pub const FF_PROFILE_MPEG4_N_BIT:                     c_int = 4;
pub const FF_PROFILE_MPEG4_SCALABLE_TEXTURE:          c_int = 5;
pub const FF_PROFILE_MPEG4_SIMPLE_FACE_ANIMATION:     c_int = 6;
pub const FF_PROFILE_MPEG4_BASIC_ANIMATED_TEXTURE:    c_int = 7;
pub const FF_PROFILE_MPEG4_HYBRID:                    c_int = 8;
pub const FF_PROFILE_MPEG4_ADVANCED_REAL_TIME:        c_int = 9;
pub const FF_PROFILE_MPEG4_CORE_SCALABLE:             c_int = 10;
pub const FF_PROFILE_MPEG4_ADVANCED_CODING:           c_int = 11;
pub const FF_PROFILE_MPEG4_ADVANCED_CORE:             c_int = 12;
pub const FF_PROFILE_MPEG4_ADVANCED_SCALABLE_TEXTURE: c_int = 13;
pub const FF_PROFILE_MPEG4_SIMPLE_STUDIO:             c_int = 14;
pub const FF_PROFILE_MPEG4_ADVANCED_SIMPLE:           c_int = 15;

pub const FF_PROFILE_JPEG2000_CSTREAM_RESTRICTION_0:  c_int = 0;
pub const FF_PROFILE_JPEG2000_CSTREAM_RESTRICTION_1:  c_int = 1;
pub const FF_PROFILE_JPEG2000_CSTREAM_NO_RESTRICTION: c_int = 2;
pub const FF_PROFILE_JPEG2000_DCINEMA_2K:             c_int = 3;
pub const FF_PROFILE_JPEG2000_DCINEMA_4K:             c_int = 4;

pub const FF_PROFILE_HEVC_MAIN:               c_int = 1;
pub const FF_PROFILE_HEVC_MAIN_10:            c_int = 2;
pub const FF_PROFILE_HEVC_MAIN_STILL_PICTURE: c_int = 3;
pub const FF_PROFILE_HEVC_REXT:               c_int = 4;

pub const FF_PROFILE_VP9_0: c_int = 0;
pub const FF_PROFILE_VP9_1: c_int = 1;
pub const FF_PROFILE_VP9_2: c_int = 2;
pub const FF_PROFILE_VP9_3: c_int = 3;

pub const FF_LEVEL_UNKNOWN: c_int = -99;

pub const FF_SUB_CHARENC_MODE_DO_NOTHING:  c_int = -1;
pub const FF_SUB_CHARENC_MODE_AUTOMATIC:   c_int = 0;
pub const FF_SUB_CHARENC_MODE_PRE_DECODER: c_int = 1;

pub const FF_CODEC_PROPERTY_LOSSLESS:        c_int = 0x00000001;
pub const FF_CODEC_PROPERTY_CLOSED_CAPTIONS: c_int = 0x00000002;

#[repr(C)]
pub struct AVCodecContext {
	pub av_class: *const AVClass,
	pub log_level_offset: c_int,
	pub codec_type: AVMediaType,
	pub codec: *const AVCodec,
	#[cfg(any(feature="ff_api_codec_name", not(feature="ff_api_codec_name_is_defined")))]
	pub codec_name: [c_char; 32],
	pub codec_id: AVCodecID,
	pub codec_tag: c_uint,
	#[cfg(feature = "ff_api_stream_codec_tag")]
	pub stream_codec_tag: c_uint,
	pub priv_data: *mut c_void,
	pub internal: *mut AVCodecInternal,
	pub opaque: *mut c_void,
	pub bit_rate: c_int,
	pub bit_rate_tolerance: c_int,
	pub global_quality: c_int,
	pub compression_level: c_int,
	pub flags: c_int,
	pub flags2: c_int,
	pub extradata: *mut uint8_t,
	pub extradata_size: c_int,
	pub time_base: AVRational,
	pub ticks_per_frame: c_int,
	pub delay: c_int,
	pub width: c_int,
	pub height: c_int,
	pub coded_width: c_int,
	pub coded_height: c_int,
	pub gop_size: c_int,
	pub pix_fmt: AVPixelFormat,
	#[cfg(feature = "ff_api_motion_est")]
	pub me_method: c_int,
	pub draw_horiz_band: Option<extern fn(*mut AVCodecContext, *const AVFrame, *mut c_int, c_int, c_int, c_int)>,
	pub get_format: Option<extern fn(*mut AVCodecContext, *const AVPixelFormat) -> AVPixelFormat>,
	pub max_b_frames: c_int,
	pub b_quant_factor: c_float,
	#[cfg(feature = "ff_api_rc_strategy")]
	pub rc_strategy: c_int,
	#[cfg(feature = "ff_api_private_opt")]
	pub b_frame_strategy: c_int,
	pub b_quant_offset: c_float,
	pub has_b_frames: c_int,
	pub mpeg_quant: c_int,
	pub i_quant_factor: c_float,
	pub i_quant_offset: c_float,
	pub lumi_masking: c_float,
	pub temporal_cplx_masking: c_float,
	pub spatial_cplx_masking: c_float,
	pub p_masking: c_float,
	pub dark_masking: c_float,
	pub slice_count: c_int,
	pub prediction_method: c_int,
	pub slice_offset: *mut c_int,
	pub sample_aspect_ratio: AVRational,
	pub me_cmp: c_int,
	pub me_sub_cmp: c_int,
	pub mb_cmp: c_int,
	pub ildct_cmp: c_int,
	pub dia_size: c_int,
	pub last_predictor_count: c_int,
	pub pre_me: c_int,
	pub me_pre_cmp: c_int,
	pub pre_dia_size: c_int,
	pub me_subpel_quality: c_int,
	#[cfg(feature = "ff_api_afd")]
	pub dtg_active_format: c_int,
	pub me_range: c_int,
	#[cfg(feature = "ff_api_quant_bias")]
	pub intra_quant_bias: c_int,
	#[cfg(feature = "ff_api_quant_bias")]
	pub inter_quant_bias: c_int,
	pub slice_flags: c_int,
	#[cfg(feature = "ff_api_xvmc")]
	pub xvmc_acceleration: c_int,
	pub mb_decision: c_int,
	pub intra_matrix: *mut uint16_t,
	pub inter_matrix: *mut uint16_t,
	pub scenechange_threshold: c_int,
	pub noise_reduction: c_int,
	#[cfg(feature = "ff_api_mpv_opt")]
	pub me_threshold: c_int,
	#[cfg(feature = "ff_api_mpv_opt")]
	pub mb_threshold: c_int,
	pub intra_dc_precision: c_int,
	pub skip_top: c_int,
	pub skip_bottom: c_int,
	#[cfg(feature = "ff_api_mpv_opt")]
	pub border_masking: c_float,
	pub mb_lmin: c_int,
	pub mb_lmax: c_int,
	pub me_penalty_compensation: c_int,
	pub bidir_refine: c_int,
	pub brd_scale: c_int,
	pub keyint_min: c_int,
	pub refs: c_int,
	pub chromaoffset: c_int,
	#[cfg(feature = "ff_api_unused_members")]
	pub scenechange_factor: c_int,
	pub mv0_threshold: c_int,
	pub b_sensitivity: c_int,
	pub color_primaries: AVColorPrimaries,
	pub color_trc: AVColorTransferCharacteristic,
	pub colorspace: AVColorSpace,
	pub color_range: AVColorRange,
	pub chroma_sample_location: AVChromaLocation,
	pub slices: c_int,
	pub field_order: AVFieldOrder,
	pub sample_rate: c_int,
	pub channels: c_int,
	pub sample_fmt: AVSampleFormat,
	pub frame_size: c_int,
	pub frame_number: c_int,
	pub block_align: c_int,
	pub cutoff: c_int,
	#[cfg(feature = "ff_api_request_channels")]
	pub request_channels: c_int,
	pub channel_layout: uint64_t,
	pub request_channel_layout: uint64_t,
	pub audio_service_type: AVAudioServiceType,
	pub request_sample_fmt: AVSampleFormat,
	#[cfg(feature = "ff_api_get_buffer")]
	pub get_buffer: Option<extern fn(*mut AVCodecContext, *mut AVFrame) -> c_int>,
	#[cfg(feature = "ff_api_get_buffer")]
	pub release_buffer: Option<extern fn(*mut AVCodecContext, *mut AVFrame)>,
	#[cfg(feature = "ff_api_get_buffer")]
	pub reget_buffer: Option<extern fn(*mut AVCodecContext, *mut AVFrame) -> c_int>,
	pub get_buffer2: Option<extern fn(*mut AVCodecContext, *mut AVFrame, c_int) -> c_int>,
	pub refcounted_frames: c_int,
	pub qcompress: c_float,
	pub qblur: c_float,
	pub qmin: c_int,
	pub qmax: c_int,
	pub max_qdiff: c_int,
	#[cfg(feature = "ff_api_mpv_opt")]
	pub rc_qsquish: c_float,
	#[cfg(feature = "ff_api_mpv_opt")]
	pub rc_qmod_amp: c_float,
	#[cfg(feature = "ff_api_mpv_opt")]
	pub rc_qmod_freq: c_int,
	pub rc_buffer_size: c_int,
	pub rc_override_count: c_int,
	pub rc_override: *mut RcOverride,
	#[cfg(feature = "ff_api_mpv_opt")]
	pub rc_eq: *const c_char,
	pub rc_max_rate: c_int,
	pub rc_min_rate: c_int,
	#[cfg(feature = "ff_api_mpv_opt")]
	pub rc_buffer_aggressivity: c_float,
	#[cfg(feature = "ff_api_mpv_opt")]
	pub rc_initial_cplx: c_float,
	pub rc_max_available_vbv_use: c_float,
	pub rc_min_vbvb_overflow_use: c_float,
	pub rc_initial_buffer_occupancy: c_int,
	pub coder_type: c_int,
	pub context_model: c_int,
	#[cfg(feature = "ff_api_mpv_opt")]
	pub lmin: c_int, 
	#[cfg(feature = "ff_api_mpv_opt")]
	pub lmax: c_int, 
	pub frame_skip_threshold: c_int,
	pub frame_skip_factor: c_int,
	pub frame_skip_exp: c_int,
	pub frame_skip_cmp: c_int,
	pub trellis: c_int,
	pub min_prediction_order: c_int,
	pub max_prediction_order: c_int,
	pub timecode_frame_start: int64_t,
	pub rtp_callback: Option<extern fn(*mut AVCodecContext, *mut c_void, c_int, c_int)>,
	pub rtp_payload_size: c_int,
	pub mv_bits: c_int,
	pub header_bits: c_int,
	pub i_tex_bits: c_int,
	pub p_tex_bits: c_int,
	pub i_count: c_int,
	pub p_count: c_int,
	pub skip_count: c_int,
	pub misc_bits: c_int,
	pub frame_bits: c_int,
	pub stats_out: *mut c_char,
	pub stats_in: *mut c_char,
	pub workaround_bugs: c_int,
	pub strict_std_compliance: c_int,
	pub error_concealment: c_int,
	pub debug: c_int,
	pub err_recognition: c_int,
	pub reordered_opaque: int64_t,
	pub hwaccel: *mut AVHWAccel,
	pub hwaccel_context: *mut c_void,
	pub error: [uint64_t; AV_NUM_DATA_POINTERS],
	pub dct_alog: c_int,
	pub idct_algo: c_int,
	pub bits_per_coded_sample: c_int,
	pub bits_per_raw_sample: c_int,
	#[cfg(feature = "ff_api_lowres")]
	pub lowres: c_int,
	#[cfg(feature = "ff_api_coded_frame")]
	pub coded_frame: *mut AVFrame,
	pub thread_count: c_int,
	pub thread_type: c_int,
	pub active_thread_type: c_int,
	pub thread_safe_callbacks: c_int,
	pub execute: extern fn(*mut AVCodecContext, extern fn(*mut AVCodecContext, *mut c_void) -> c_int, *mut c_void, *mut c_int, c_int, c_int) -> c_int,
	pub execute2: extern fn(*mut AVCodecContext, extern fn(*mut AVCodecContext, *mut c_void, c_int, c_int) -> c_int, *mut c_void, *mut c_int, c_int) -> c_int,
	#[cfg(feature = "ff_api_thread_opaque")]
	pub thread_opaque: *mut c_void,
	pub nsse_weight: c_int,
	pub profile: c_int,
	pub level: c_int,
	pub skip_loop_filter: AVDiscard,
	pub skip_idct: AVDiscard,
	pub skip_frame: AVDiscard,
	pub subtitle_header: *mut uint8_t,
	pub subtitle_header_size: c_int,
	#[cfg(feature = "ff_api_error_rate")]
	pub error_rate: c_int,
	#[cfg(feature = "ff_api_codec_pkt")]
	pub pkt: *mut AVPacket,
	pub vbv_delay: uint64_t,
	pub side_data_only_packets: c_int,
	pub initial_padding: c_int,
	pub framerate: AVRational,
	pub sw_pix_fmt: AVPixelFormat,
	pub pkt_timebase: AVRational,
	pub codec_descriptor: *const AVCodecDescriptor,
	#[cfg(not(feature = "ff_api_lowres"))]
	pub lowres: c_int,
	pub pts_correction_num_faulty_pts: int64_t,
	pub pts_correction_num_faulty_dts: int64_t,
	pub pts_correction_last_pts: int64_t,
	pub pts_correction_last_dts: int64_t,
	pub sub_charenc: *mut c_char,
	pub sub_charenc_mode: c_int,
	pub skip_alpha: c_int,
	pub seek_preroll: c_int,
	#[cfg(not(feature = "ff_api_debug_mv"))]
	pub debug_mv: c_int,
	pub chroma_intra_matrix: *mut uint16_t,
	pub dump_separator: *mut uint8_t,
	pub codec_whitelist: *mut c_char,
	pub properties: c_uint,
}

#[derive(Debug)]
#[repr(C)]
pub struct AVProfile {
	pub profile: c_int,
	pub name: *const c_char,
}

pub type AVCodecDefault = c_void;

#[repr(C)]
pub struct AVCodec {
	pub name: *const c_char,
	pub long_name: *const c_char,
	pub kind: AVMediaType,
	pub id: AVCodecID,
	pub capabilities: c_int,
	pub supported_framerates: *const AVRational,
	pub pix_fmts: *const AVPixelFormat,
	pub supported_samplerates: *const c_int,
	pub sample_fmts: *const AVSampleFormat,
	pub channel_layouts: *const uint64_t,
	pub priv_class: *const AVClass,
	pub max_lowres: uint8_t,
	pub profiles: *const AVProfile,

	pub priv_data_size: c_int,
	pub next: *mut AVCodec,
	pub init_thread_copy: Option<extern fn(*mut AVCodecContext) -> c_int>,
	pub update_thread_context: extern fn(*mut AVCodecContext, *const AVCodecContext) -> c_int,
	pub defaults: *const AVCodecDefault,
	pub init_static_data: extern fn(*mut AVCodec),
	pub init: extern fn(*mut AVCodecContext) -> c_int,
	pub encode_sub: extern fn(*mut AVCodecContext, *mut uint8_t, c_int, *const AVSubtitle) -> c_int,
	pub encode2: extern fn(*mut AVCodecContext, *mut AVPacket, *const AVFrame, *mut c_int) -> c_int,
	pub decode: extern fn(*mut AVCodecContext, *mut c_void, *mut c_int, *mut AVPacket) -> c_int,
	pub close: extern fn(*mut AVCodecContext) -> c_int,
	pub flush: extern fn(*mut AVCodecContext),
	pub caps_internal: c_int,
}

pub const AV_HWACCEL_FLAG_IGNORE_LEVEL:     c_int = 1 << 0;
pub const AV_HWACCEL_FLAG_ALLOW_HIGH_DEPTH: c_int = 1 << 1;

pub type MpegEncContext = c_void;

#[repr(C)]
pub struct AVHWAccel {
	pub name: *const c_char,
	pub kind: AVMediaType,
	pub id: AVCodecID,
	pub pix_fmt: AVPixelFormat,
	pub capabilities: c_int,

	pub next: *mut AVHWAccel,
	pub alloc_frame: extern fn(*mut AVCodecContext, *mut AVFrame) -> c_int,
	pub start_frame: extern fn(*mut AVCodecContext, *const uint8_t, uint32_t) -> c_int,
	pub decode_slice: extern fn(*mut AVCodecContext, *const uint8_t, uint32_t) -> c_int,
	pub end_frame: extern fn(*mut AVCodecContext) -> c_int,
	pub frame_priv_data_size: c_int,
	pub decode_mb: extern fn(*mut MpegEncContext),
	pub init: extern fn(*mut AVCodecContext) -> c_int,
	pub uninit: extern fn(*mut AVCodecContext) -> c_int,
	pub priv_data_size: c_int,
}

#[derive(Debug)]
#[repr(C)]
pub struct AVPicture {
	pub data: [*mut uint8_t; AV_NUM_DATA_POINTERS],
	pub linesize: [c_int; AV_NUM_DATA_POINTERS],
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
#[repr(C)]
pub enum AVSubtitleType {
	SUBTITLE_NONE,
	SUBTITLE_BITMAP,
	SUBTITLE_TEXT,
	SUBTITLE_ASS,
}

pub const AV_SUBTITLE_FLAG_FORCED: c_int = 0x00000001;

#[derive(Debug)]
#[repr(C)]
pub struct AVSubtitleRect {
	pub x: c_int,
	pub y: c_int,
	pub w: c_int,
	pub h: c_int,
	pub nb_colors: c_int,
	pub pict: AVPicture,
	pub kind: AVSubtitleType,
	pub text: *mut c_char,
	pub ass: *mut c_char,
	pub flags: c_int,
}

#[derive(Debug)]
#[repr(C)]
pub struct AVSubtitle {
	pub format: uint16_t,
	pub start_display_time: uint32_t,
	pub end_display_time: uint32_t,
	pub num_rects: c_uint,
	pub rects: *mut *mut AVSubtitleRect,
	pub pts: int64_t,
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
#[repr(C)]
pub enum AVPictureStructure {
	AV_PICTURE_STRUCTURE_UNKNOWN,
	AV_PICTURE_STRUCTURE_TOP_FIELD,
	AV_PICTURE_STRUCTURE_BOTTOM_FIELD,
	AV_PICTURE_STRUCTURE_FRAME,
}

pub const AV_PARSER_PTS_NB: usize = 4;

pub const PARSER_FLAG_COMPLETE_FRAMES: c_int = 0x0001;
pub const PARSER_FLAG_ONCE:            c_int = 0x0002;
pub const PARSER_FLAG_FETCHED_OFFSET:  c_int = 0x0004;
pub const PARSER_FLAG_USE_CODEC_TS:    c_int = 0x1000;

#[derive(Debug)]
#[repr(C)]
pub struct AVCodecParserContext {
	pub priv_data: *mut c_void,
	pub parser: *mut AVCodecParser,
	pub frame_offset: int64_t,
	pub cur_offset: int64_t,
	pub next_frame_offset: int64_t,
	pub pict_type: c_int,
	pub repeat_pict: c_int,
	pub pts: int64_t,
	pub dts: int64_t,

	pub last_pts: int64_t,
	pub last_dts: int64_t,
	pub fetch_timestamp: c_int,
	pub cur_frame_start_index: c_int,
	pub cur_frame_offset: [int64_t; AV_PARSER_PTS_NB],
	pub cur_frame_pts: [int64_t; AV_PARSER_PTS_NB],
	pub cur_frame_dts: [int64_t; AV_PARSER_PTS_NB],
	pub flags: c_int,
	pub offset: int64_t,
	pub cur_frame_end: [int64_t; AV_PARSER_PTS_NB],
	pub key_frame: c_int,
	pub convergence_duration: int64_t,
	pub dts_sync_point: c_int,
	pub dts_ref_dts_delta: c_int,
	pub pts_dts_delta: c_int,
	pub cur_frame_pos: [int64_t; AV_PARSER_PTS_NB],
	pub pos: int64_t,
	pub last_pos: int64_t,
	pub duration: c_int,
	pub field_order: AVFieldOrder,
	pub picture_structure: AVPictureStructure,
	pub output_picture_number: c_int,
	pub width: c_int,
	pub height: c_int,
	pub coded_width: c_int,
	pub coded_height: c_int,
	pub format: c_int,
}

#[repr(C)]
pub struct AVCodecParser {
	pub codec_ids: [c_int; 5],
	pub priv_data_size: c_int,
	pub parser_init: extern fn(*mut AVCodecParserContext) -> c_int,
	pub parser_parse: extern fn(*mut AVCodecParserContext, *mut AVCodecContext, *const *const uint8_t, *mut c_int, *const uint8_t, c_int) -> c_int,
	pub parser_close: extern fn(*mut AVCodecParserContext),
	pub split: extern fn(*mut AVCodecContext, *const uint8_t, c_int) -> c_int,
	pub next: *mut AVCodecParser,
}

#[derive(Debug)]
#[repr(C)]
pub struct AVBitStreamFilterContext {
	pub priv_data: *mut c_void,
	pub filter: *mut AVBitStreamFilter,
	pub parser: *mut AVCodecParserContext,
	pub next: *mut AVBitStreamFilterContext,
}

#[repr(C)]
pub struct AVBitStreamFilter {
	pub name: *const c_char,
	pub priv_data_size: c_int,
	pub filter: extern fn(*mut AVBitStreamFilterContext, *mut AVCodecContext, *const c_char, *mut *mut uint8_t, *mut c_int, *const uint8_t, c_int, c_int) -> c_int,
	pub close: extern fn(*mut AVBitStreamFilterContext),
	pub next: *mut AVBitStreamFilter,
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
#[repr(C)]
pub enum AVLockOp {
	AV_LOCK_CREATE,
	AV_LOCK_OBTAIN,
	AV_LOCK_RELEASE,
	AV_LOCK_DESTROY,
}

extern {
	pub fn av_codec_get_pkt_timebase(avctx: *const AVCodecContext) -> AVRational;
	pub fn av_codec_set_pkt_timebase(avctx: *mut AVCodecContext, val: AVRational);

	pub fn av_codec_get_codec_descriptor(avctx: *const AVCodecContext) -> *const AVCodecDescriptor;
	pub fn av_codec_set_codec_descriptor(avctx: *mut AVCodecContext, val: *const AVCodecDescriptor);

	pub fn av_codec_get_codec_properties(avctx: *const AVCodecContext) -> c_uint;

	pub fn av_codec_get_lowres(avctx: *const AVCodecContext) -> c_int;
	pub fn av_codec_set_lowres(avctx: *mut AVCodecContext, val: c_int);

	pub fn av_codec_get_seek_preroll(avctx: *const AVCodecContext) -> c_int;
	pub fn av_codec_set_seek_preroll(avctx: *mut AVCodecContext, val: c_int);

	pub fn av_codec_get_max_lowres(codec: *const AVCodec) -> c_int;

	pub fn av_codec_next(c: *const AVCodec) -> *mut AVCodec;

	pub fn avcodec_version() -> c_uint;
	pub fn avcodec_configuration() -> *const c_char;
	pub fn avcodec_license() -> *const c_char;

	pub fn avcodec_register(codec: *mut AVCodec);
	pub fn avcodec_register_all();

	pub fn avcodec_alloc_context3(codec: *const AVCodec) -> *mut AVCodecContext;
	pub fn avcodec_free_context(avctx: *mut *mut AVCodecContext);
	pub fn avcodec_get_context_defaults3(s: *mut AVCodecContext, codec: *const AVCodec) -> c_int;

	pub fn avcodec_get_class() -> *const AVClass;
	pub fn avcodec_get_frame_class() -> *const AVClass;
	pub fn avcodec_get_subtitle_rect_class() -> *const AVClass;

	pub fn avcodec_copy_context(dest: *mut AVCodecContext, src: *const AVCodecContext) -> c_int;

	pub fn avcodec_alloc_frame() -> *mut AVFrame;
	pub fn avcodec_get_frame_defaults(frame: *mut AVFrame);
	pub fn avcodec_free_frame(frame: *mut *mut AVFrame);

	pub fn avcodec_open2(avctx: *mut AVCodecContext, codec: *const AVCodec, options: *mut *mut AVDictionary) -> c_int;
	pub fn avcodec_close(avctx: *mut AVCodecContext) -> c_int;

	pub fn avsubtitle_free(sub: *mut AVSubtitle);

	pub fn av_init_packet(pkt: *mut AVPacket);
	pub fn av_new_packet(pkt: *mut AVPacket, size: c_int) -> c_int;
	pub fn av_shrink_packet(pkt: *mut AVPacket, size: c_int);
	pub fn av_grow_packet(pkt: *mut AVPacket, grow_by: c_int) -> c_int;
	pub fn av_packet_from_data(pkt: *mut AVPacket, data: *mut uint8_t, size: c_int) -> c_int;
	pub fn av_dup_packet(pkt: *mut AVPacket) -> c_int;
	pub fn av_copy_packet(dst: *mut AVPacket, src: *const AVPacket) -> c_int;
	pub fn av_copy_packet_side_data(dst: *mut AVPacket, src: *const AVPacket) -> c_int;
	pub fn av_free_packet(pkt: *mut AVPacket);
	pub fn av_packet_new_side_data(pkt: *mut AVPacket, kind: AVPacketSideDataType, size: c_int) -> *mut uint8_t;
	pub fn av_packet_shrink_side_data(pkt: *mut AVPacket, kind: AVPacketSideDataType, size: c_int) -> c_int;
	pub fn av_packet_get_side_data(pkt: *mut AVPacket, kind: AVPacketSideDataType, size: *mut c_int) -> *mut uint8_t;
	pub fn av_packet_merge_side_data(pkt: *mut AVPacket) -> c_int;
	pub fn av_packet_split_side_data(pkt: *mut AVPacket) -> c_int;
	pub fn av_packet_side_data_name(kind: AVPacketSideDataType) -> *const c_char;
	pub fn av_packet_pack_dictionary(dict: *mut AVDictionary, size: *mut c_int) -> *mut uint8_t;
	pub fn av_packet_unpack_dictionary(data: *const uint8_t, size: c_int, dict: *mut *mut AVDictionary) -> c_int;
	pub fn av_packet_free_side_data(pkt: *mut AVPacket);
	pub fn av_packet_ref(dst: *mut AVPacket, src: *const AVPacket) -> c_int;
	pub fn av_packet_unref(pkt: *mut AVPacket);
	pub fn av_packet_move_Ref(dst: *mut AVPacket, src: *mut AVPacket);
	pub fn av_packet_copy_props(dst: *mut AVPacket, src: *const AVPacket) -> c_int;
	pub fn av_packet_rescale_ts(pkt: *mut AVPacket, tb_src: AVRational, tb_dst: AVRational);

	pub fn avcodec_find_decoder(id: AVCodecID) -> *mut AVCodec;
	pub fn avcodec_find_decoder_by_name(name: *const c_char) -> *mut AVCodec;

	pub fn avcodec_default_get_buffer(s: *mut AVCodecContext, frame: *mut AVFrame, flags: c_int) -> c_int;

	pub fn avcodec_align_dimensions(s: *mut AVCodecContext, width: *mut c_int, height: *mut c_int);
	pub fn avcodec_align_dimensions2(s: *mut AVCodecContext, width: *mut c_int, height: *mut c_int, linesize_align: *mut c_int);

	pub fn avcodec_enum_to_chroma_pos(xpos: *mut c_int, ypos: *mut c_int, pos: AVChromaLocation) -> c_int;
	pub fn avcodec_chroma_pos_to_enum(xpos: c_int, ypos: c_int) -> AVChromaLocation;

	pub fn avcodec_decode_audio4(avctx: *mut AVCodecContext, frame: *mut AVFrame, got_frame_ptr: *mut c_int, avpkt: *const AVPacket) -> c_int;
	pub fn avcodec_decode_video2(avctx: *mut AVCodecContext, picture: *mut AVFrame, got_picture_ptr: *mut c_int, avpkt: *const AVPacket) -> c_int;
	pub fn avcodec_decode_subtitle2(avctx: *mut AVCodecContext, sub: *mut AVSubtitle, got_sub_ptr: *mut c_int, avpkt: *const AVPacket) -> c_int;

	pub fn av_parser_next(c: *const AVCodecParser) -> *mut AVCodecParser;
	pub fn av_register_codec_parser(parser: *mut AVCodecParser);
	pub fn av_parser_init(codec_id: c_int) -> *mut AVCodecParserContext;
	pub fn av_parser_parse2(s: *mut AVCodecParserContext, avctx: *mut AVCodecContext, poutbuf: *mut *mut uint8_t, poutbuf_size: *mut c_int, buf: *const uint8_t, buf_size: c_int, pts: int64_t, dts: int64_t, pos: int64_t) -> c_int;
	pub fn av_parser_change(s: *mut AVCodecParserContext, avctx: *mut AVCodecContext, poutbuf: *mut *mut uint8_t, poutbuf_size: *mut c_int, buf: *const uint8_t, buf_size: c_int, keyframe: c_int) -> c_int;
	pub fn av_parser_close(s: *mut AVCodecParserContext);

	pub fn avcodec_find_encoder(id: AVCodecID) -> *mut AVCodec;
	pub fn avcodec_find_encoder_by_name(name: *const c_char) -> *mut AVCodec;
	pub fn avcodec_encode_audio2(avctx: *mut AVCodecContext, avpkt: *mut AVPacket, frame: *const AVFrame, got_packet_ptr: *mut c_int) -> c_int;
	pub fn avcodec_encode_video2(avctx: *mut AVCodecContext, avpkt: *mut AVPacket, frame: *const AVFrame, got_packet_ptr: *mut c_int) -> c_int;
	pub fn avcodec_encode_subtitle(avctx: *mut AVCodecContext, buf: *mut uint8_t, buf_size: c_int, sub: *const AVSubtitle) -> c_int;

	pub fn avpicture_alloc(picture: *mut AVPicture, pix_fmt: AVPixelFormat, width: c_int, height: c_int) -> c_int;
	pub fn avpicture_free(picture: *mut AVPicture);
	pub fn avpicture_fill(picture: *mut AVPicture, ptr: *const uint8_t, pix_fmt: AVPixelFormat, width: c_int, height: c_int) -> c_int;
	pub fn avpicture_layout(src: *const AVPicture, pix_fmt: AVPixelFormat, width: c_int, height: c_int, dest: *mut c_uchar, dest_size: c_int) -> c_int;
	pub fn avpicture_get_size(pix_fmt: AVPixelFormat, width: c_int, height: c_int) -> c_int;

	pub fn av_picture_copy(dst: *mut AVPicture, src: *const AVPicture, pix_fmt: AVPixelFormat, width: c_int, height: c_int);
	pub fn av_picture_crop(dst: *mut AVPicture, src: *const AVPicture, pix_fmt: AVPixelFormat, top_band: c_int, left_band: c_int) -> c_int;
	pub fn av_picture_pad(dst: *mut AVPicture, src: *const AVPicture, height: c_int, width: c_int, pix_fmt: AVPixelFormat, padtop: c_int, padbottom: c_int, padleft: c_int, padright: c_int, color: *mut c_int) -> c_int;

	pub fn avcodec_get_chroma_sub_sample(pix_fmt: AVPixelFormat, h_shift: *mut c_int, v_shift: *mut c_int);
	pub fn avcodec_pix_fmt_to_codec_tag(pix_fmt: AVPixelFormat) -> c_uint;
	pub fn avcodec_get_pix_fmt_loss(dst_pix_fmt: AVPixelFormat, src_pix_fmt: AVPixelFormat, has_alpha: c_int) -> c_int;
	pub fn avcodec_find_best_pix_fmt_of_list(pix_fmt_list: *const AVPixelFormat, src_pix_fmt: AVPixelFormat, has_alpha: c_int, loss_ptr: *mut c_int) -> AVPixelFormat;
	pub fn avcodec_find_best_pix_fmt_of_2(dst_pix_fmt1: AVPixelFormat, dst_pix_fmt2: AVPixelFormat, src_pix_fmt: AVPixelFormat, has_alpha: c_int, loss_ptr: *mut c_int) -> AVPixelFormat;
	pub fn avcodec_find_best_pix_fmt2(dst_pix_fmt1: AVPixelFormat, dst_pix_fmt2: AVPixelFormat, src_pix_fmt: AVPixelFormat, has_alpha: c_int, loss_ptr: *mut c_int) -> AVPixelFormat;
	pub fn avcodec_default_get_format(s: *mut AVCodecContext, fmt: *const AVPixelFormat) -> AVPixelFormat;

	pub fn av_get_codec_tag_string(buf: *mut c_char, buf_size: size_t, codec_tag: c_uint) -> size_t;
	pub fn avcodec_string(buf: *mut c_char, buf_size: c_int, enc: *mut AVCodecContext, encode: c_int);
	pub fn av_get_profile_name(codec: *const AVCodec, profile: c_int) -> *const c_char;
	pub fn avcodec_default_execute(c: *mut AVCodecContext, func: extern fn(*mut AVCodecContext, *mut c_void) -> c_int, arg: *mut c_void, ret: *mut c_int, count: c_int, size: c_int) -> c_int;
	pub fn avcodec_default_execute2(c: *mut AVCodecContext, func: extern fn(*mut AVCodecContext, *mut c_void, c_int, c_int) -> c_int, arg: *mut c_void, ret: *mut c_int, count: c_int) -> c_int;
	pub fn avcodec_fill_audio_frame(frame: *mut AVFrame, nb_channels: c_int, sample_fmt: AVSampleFormat, buf: *const uint8_t, buf_size: c_int, align: c_int) -> c_int;
	pub fn avcodec_flush_buffers(avctx: *mut AVCodecContext);
	pub fn av_get_bits_per_sample(codec_id: AVCodecID) -> c_int;
	pub fn av_get_pcm_codec(fmt: AVSampleFormat, be: c_int) -> AVCodecID;
	pub fn av_get_exact_bits_per_sample(codec_id: AVCodecID) -> c_int;
	pub fn av_get_audio_frame_duration(avctx: *mut AVCodecContext, frame_bytes: c_int) -> c_int;

	pub fn av_register_bitstream_filter(bsf: *mut AVBitStreamFilter);
	pub fn av_bitstream_filter_init(name: *const c_char) -> *mut AVBitStreamFilterContext;
	pub fn av_bitstream_filter_filter(bsfc: *mut AVBitStreamFilterContext, avctx: *mut AVCodecContext, args: *const c_char, poutbuf: *mut *mut uint8_t, poutbuf_size: *mut c_int, buf: *const uint8_t, buf_size: c_int, keyframe: c_int) -> c_int;
	pub fn av_bitstream_filter_close(bsf: *mut AVBitStreamFilterContext);
	pub fn av_bitstream_filter_next(f: *const AVBitStreamFilter) -> *mut AVBitStreamFilter;

	pub fn av_fast_padded_malloc(ptr: *mut c_void, size: *mut c_uint, min_size: size_t);
	pub fn av_fast_padded_mallocz(ptr: *mut c_void, size: *mut c_uint, min_size: size_t);
	pub fn av_xiphlacing(s: *mut c_uchar, v: c_uint) -> c_uint;

	pub fn av_register_hwaccel(hwaccel: *mut AVHWAccel);
	pub fn av_hwaccel_next(hwaccel: *const AVHWAccel) -> *mut AVHWAccel;

	pub fn av_lockmgr_register(cb: extern fn(*mut *mut c_void, AVLockOp) -> c_int) -> c_int;

	pub fn avcodec_get_type(codec_id: AVCodecID) -> AVMediaType;
	pub fn avcodec_get_name(id: AVCodecID) -> *const c_char;
	pub fn avcodec_is_open(s: *mut AVCodecContext) -> c_int;
	pub fn av_codec_is_encoder(codec: *const AVCodec) -> c_int;
	pub fn av_codec_is_decoder(codec: *const AVCodec) -> c_int;

	pub fn avcodec_descriptor_get(id: AVCodecID) -> *const AVCodecDescriptor;
	pub fn avcodec_descriptor_next(prev: *const AVCodecDescriptor) -> *const AVCodecDescriptor;
	pub fn avcodec_descriptor_get_by_name(name: *const c_char) -> *const AVCodecDescriptor;
}
