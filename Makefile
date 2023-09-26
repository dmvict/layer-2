word-slash = $(word $2,$(subst /, ,$1))

# Copy files.
#
# Usage:
#	make copy [repo=(|[repo])]

copy:
ifeq ($(strip $(call word-slash,$(repo),2)),)
	[ -d "owner/$(repo)" ] && cp -R owner/$(repo)/* ./ || echo -e "\033[0;31mSource directory \"$(repo)\" does not exists\033[0m"
else
	[ -d "owner/$(call word-slash,$(repo),2)" ] && cp -R owner/$(call word-slash,$(repo),2)/* ./ \
		|| echo -e "\033[0;31mSource directory \"$(call word-slash,$(repo),2)\" does not exists\033[0m"
endif

# Save artifacts.
save:
	git config --local user.email "conan@Kings-MacBook-Pro.local"
	git config --local user.name "King I"
	mkdir -p dist
ifeq (./Cargo.toml,$(wildcard ./Cargo.toml))
	$(eval msg:=$(shell v=$$(cat Cargo.toml | grep 'version =' -m 1 | awk -F ' ' '{print $$3}' | awk -F '"' '{print $$2}') && \
		[[ $$v != "" ]] && \
		echo v$$v || echo Save workspace apps))
	for path in $$(find target/x86_64-unknown-linux-musl/release/ -maxdepth 1 -type f -executable) ; do \
		name=$$(basename $$path) \
		cp -R $$path dist/$$name ; \
	done
endif
ifeq (./package.json,$(wildcard ./package.json))
	$(eval msg:=v$(shell cat package.json | jq '.version' --raw-output))
endif
ifeq ($(wildcard ./Cargo.toml),$(wildcard ./package.json))
	echo -e "\033[0;31mUnknown build target to save\033[0m"
endif
	git add -f dist
	git commit -m "${msg}"
	git config --local --remove-section user

.PHONY: \
	copy \
	save \
