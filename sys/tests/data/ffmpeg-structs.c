#include <libavcodec/avcodec.h>

int main()
{
	#define p(_struct, _member) { \
		_struct* p = 0; \
		unsigned _mysizeof = (unsigned)((size_t)( (&p->_member) + 1 ) - (size_t)( &p->_member )); \
		printf("[" #_struct "::" #_member " @ %u-%u]\n", (unsigned)offsetof(_struct, _member), _mysizeof); \
	}

	p(AVCodecContext, av_class);
	p(AVCodecContext, codec_id);
	p(AVCodecContext, bit_rate);
	p(AVCodecContext, bit_rate_tolerance);
	p(AVCodecContext, width);
	p(AVCodecContext, height);
	p(AVCodecContext, coded_width);
	p(AVCodecContext, pix_fmt);
	p(AVCodecContext, sample_rate);
	p(AVCodecContext, channels);
	p(AVCodecContext, sample_fmt);
	p(AVCodecContext, frame_size);
	p(AVCodecContext, frame_number);
	p(AVCodecContext, block_align);

	return 0;
}
