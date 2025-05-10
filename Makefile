BUILD=build
THIRDPARTY=thirdparty
CRATES=crates
SRC=src

ARGS = "-lc $(BUILD)/nob.o $(BUILD)/flag.o"
RFLAGS = --edition 2021 -g -C opt-level=z -C link-args=$(ARGS) -C panic="abort"

.PHONY: all projects

all: $(BUILD) $(BUILD)/nob.o $(BUILD)/flag.o projects

$(BUILD):
	@mkdir -p $(BUILD)

$(BUILD)/nob.o: $(THIRDPARTY)/nob.h
	gcc -g -x c -DNOB_IMPLEMENTATION -c $(THIRDPARTY)/nob.h -o $(BUILD)/nob.o

$(BUILD)/flag.o: $(THIRDPARTY)/flag.h
	gcc -g -x c -DFLAG_IMPLEMENTATION -c $(THIRDPARTY)/flag.h -o $(BUILD)/flag.o

projects: 0_wcr 1_rjsonp

0_wcr: $(SRC)/0_wc.rs $(BUILD)/nob.o $(BUILD)/flag.o
	rustc $(RFLAGS) $< -o $(BUILD)/$@

1_rjsonp: $(SRC)/1_json_parser.rs $(BUILD)/nob.o $(BUILD)/flag.o
	rustc $(RFLAGS) $< -o $(BUILD)/$@

r-a: rust-project.json.template
	@echo "Generating rust-project.json from rust-project.json.template ..."
	@envsubst < rust-project.json.template > rust-project.json
	@echo "Done."

vsset:
	@mkdir -p .vscode/
	@echo '{' > .vscode/settings.json
	@echo '  "rust-analyzer.linkedProjects": [' >> .vscode/settings.json
	@echo '    "rust-project.json"' >> .vscode/settings.json
	@echo '  ]' >> .vscode/settings.json
	@echo '}' >> .vscode/settings.json
	@echo "Generated .vscode/settings.json"



clean:
	rm -rf $(BUILD)/ rust-project.json
