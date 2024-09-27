.PHONY : all

all:
	@cd kernel && make -s all

%:
	@cd kernel && make -s $@