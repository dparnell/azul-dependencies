#
#  fontconfig/fc-query/Makefile.am
#
#  Copyright © 2003 Keith Packard
#
#  Permission to use, copy, modify, distribute, and sell this software and its
#  documentation for any purpose is hereby granted without fee, provided that
#  the above copyright notice appear in all copies and that both that
#  copyright notice and this permission notice appear in supporting
#  documentation, and that the name of the author(s) not be used in
#  advertising or publicity pertaining to distribution of the software without
#  specific, written prior permission.  The authors make no
#  representations about the suitability of this software for any purpose.  It
#  is provided "as is" without express or implied warranty.
#
#  THE AUTHOR(S) DISCLAIMS ALL WARRANTIES WITH REGARD TO THIS SOFTWARE,
#  INCLUDING ALL IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS, IN NO
#  EVENT SHALL THE AUTHOR(S) BE LIABLE FOR ANY SPECIAL, INDIRECT OR
#  CONSEQUENTIAL DAMAGES OR ANY DAMAGES WHATSOEVER RESULTING FROM LOSS OF USE,
#  DATA OR PROFITS, WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE OR OTHER
#  TORTIOUS ACTION, ARISING OUT OF OR IN CONNECTION WITH THE USE OR
#  PERFORMANCE OF THIS SOFTWARE.

bin_PROGRAMS=fc-validate

DOC2MAN = docbook2man

FC_VALIDATE_SRC=${top_srcdir}/fc-validate

SGML = ${FC_VALIDATE_SRC}/fc-validate.sgml

AM_CPPFLAGS=-I${top_srcdir} $(FREETYPE_CFLAGS) $(WARN_CFLAGS)

BUILT_MANS=fc-validate.1

if ENABLE_DOCS
man_MANS=${BUILT_MANS}
endif

EXTRA_DIST=fc-validate.sgml $(BUILT_MANS)

CLEANFILES =

fc_validate_LDADD = ${top_builddir}/src/libfontconfig.la $(FREETYPE_LIBS)

if USEDOCBOOK

${man_MANS}: ${SGML}
	$(AM_V_GEN) $(RM) $@; \
	$(DOC2MAN) ${SGML}; \
	$(RM) manpage.*

all-local: $(man_MANS)

CLEANFILES += $(man_MANS)
else
all-local:
endif

-include $(top_srcdir)/git.mk
