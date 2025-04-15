SHELL := /bin/bash

# GHIDRA_SRC points to your local Ghidra repo under pcode-generator/ghidra
# We default to ghidra inside the current directory (CURDIR).
GHIDRA_SRC ?= $(CURDIR)/ghidra

# The subdirectory that provides the makefile target for sleigh_opt
SLEIGH_OPT_DIR    := $(GHIDRA_SRC)/Ghidra/Features/Decompiler/src/decompile/cpp
# The resulting sleigh_opt tool after building in the above directory
SLEIGH_OPT_TARGET := $(SLEIGH_OPT_DIR)/sleigh_opt

# The official x86-64 SLEIGH spec within your local Ghidra repo
SLASPEC := $(GHIDRA_SRC)/Ghidra/Processors/x86/data/languages/x86-64.slaspec

# Output compiled .sla file name (binary SLEIGH spec)
OUT_SLA ?= x86-64.sla

.PHONY: all sleigh_opt compile_sla clean

## "make all" will build sleigh_opt (if not built) then compile_sla
all: compile_sla

## 1) Build the sleigh_opt tool by calling 'make sleigh_opt' in Ghidra's decompiler/cpp
sleigh_opt:
	@echo "[*] Building sleigh_opt inside $(SLEIGH_OPT_DIR) ..."
	cd $(SLEIGH_OPT_DIR) && make sleigh_opt
	@echo "[*] Done building sleigh_opt."

## 2) Compile the .slaspec -> binary .sla
compile_sla: sleigh_opt
	@echo "[*] Compiling SLEIGH spec from '$(SLASPEC)'..."
	$(SLEIGH_OPT_TARGET) $(SLASPEC)
	@echo "[*] Done. Created compiled SLA file: '$(OUT_SLA)'"

## 3) Clean up
## Removes the compiled SLA and also calls 'make clean' in the Decompiler/cpp directory
clean:
	@echo "[*] Removing '$(OUT_SLA)'..."
	rm -f $(OUT_SLA)
	@echo "[*] Cleaning up in $(SLEIGH_OPT_DIR)..."
	cd $(SLEIGH_OPT_DIR) && make clean
