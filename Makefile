NAME=pbrain-gomoku-ai

DEBUG_TARGET	:= ./target/debug/gomoku
RELEASE_TARGET	:= ./target/release/gomoku

ifeq ($(BUILD_MODE), DEBUG)
    TARGET := $(DEBUG_TARGET)
else
    TARGET := $(RELEASE_TARGET)
endif

all: $(NAME)

$(NAME): $(TARGET)
	ln -sf $< $@

$(RELEASE_TARGET):
	cargo build --release

$(DEBUG_TARGET):
	cargo build

.PHONY: $(RELEASE_TARGET) $(DEBUG_TARGET)
