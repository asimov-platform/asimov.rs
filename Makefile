all:

clean:
	@rc=0; \
	for dir in dart js python ruby rust; do \
		if [ -d $$dir ]; then \
			$(MAKE) -C $$dir clean || rc=$$?; \
		fi; \
	done; \
	exit $$rc

maintainer-clean:
	@rc=0; \
	for dir in dart js python ruby rust; do \
		if [ -d $$dir ]; then \
			$(MAKE) -C $$dir maintainer-clean || rc=$$?; \
		fi; \
	done; \
	exit $$rc

.PHONY: all clean maintainer-clean
.SECONDARY:
.SUFFIXES:
