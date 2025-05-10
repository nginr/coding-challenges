BUILD=build
THIRDPARTY=thirdparty
CRATES=crates
SRC=src

ARGS = "-lc $(BUILD)/nob.o $(BUILD)/flag.o -lm"
RFLAGS = --edition 2021 -g -C opt-level=1 -C debuginfo=2 -C link-args=$(ARGS) -C panic="abort"

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

vlaunch:
	@mkdir -p .vscode/
	@echo '{' > .vscode/launch.json
	@echo '    "version": "0.2.0",' >> .vscode/launch.json
	@echo '    "configurations": [' >> .vscode/launch.json
	@echo '        {' >> .vscode/launch.json
	@echo '            "name": "Debug Rust",' >> .vscode/launch.json
	@echo '            "type": "cppdbg",' >> .vscode/launch.json
	@echo '            "request": "launch",' >> .vscode/launch.json
	@echo '            "program": "${workspaceFolder}/build/1_rjsonp",' >> .vscode/launch.json
	@echo '            "args": ["${workspaceFolder}/tests/1_json_parser/step2/invalid.json"],' >> .vscode/launch.json
	@echo '            "cwd": "${workspaceFolder}",' >> .vscode/launch.json
	@echo '            "stopAtEntry": true,' >> .vscode/launch.json
	@echo '            "MIMode": "gdb",' >> .vscode/launch.json
	@echo '            "setupCommands": [' >> .vscode/launch.json
	@echo '                {' >> .vscode/launch.json
	@echo '                    "description": "Enable pretty-printing for gdb",' >> .vscode/launch.json
	@echo '                    "text": "-enable-pretty-printing",' >> .vscode/launch.json
	@echo '                    "ignoreFailures": true' >> .vscode/launch.json
	@echo '                },' >> .vscode/launch.json
	@echo '                {' >> .vscode/launch.json
	@echo '                    "description": "Set Disassembly Flavor to Intel",' >> .vscode/launch.json
	@echo '                    "text": "-gdb-set disassembly-flavor intel",' >> .vscode/launch.json
	@echo '                    "ignoreFailures": true' >> .vscode/launch.json
	@echo '                }' >> .vscode/launch.json
	@echo '            ]' >> .vscode/launch.json
	@echo '        }' >> .vscode/launch.json
	@echo '    ]' >> .vscode/launch.json
	@echo '}' >> .vscode/launch.json
	@echo "Generated .vscode/launch.json"

tests: t0_wc t1_rjsonp

t0_wc: 0_wcr
	$(BUILD)/0_wcr -c tests/0_wc/test.txt
	$(BUILD)/0_wcr -l tests/0_wc/test.txt
	$(BUILD)/0_wcr tests/0_wc/test.txt

t1_rjsonp: 1_rjsonp
	$(BUILD)/1_rjsonp tests/1_json_parser/step1/valid.json
	$(BUILD)/1_rjsonp tests/1_json_parser/step1/invalid.json || (exit 0)
	$(BUILD)/1_rjsonp tests/1_json_parser/step2/valid.json
	$(BUILD)/1_rjsonp tests/1_json_parser/step2/invalid.json || (exit 0)
	$(BUILD)/1_rjsonp tests/1_json_parser/step2/valid2.json
	$(BUILD)/1_rjsonp tests/1_json_parser/step2/invalid2.json || (exit 0)
	$(BUILD)/1_rjsonp tests/1_json_parser/step3/valid.json
	$(BUILD)/1_rjsonp tests/1_json_parser/step3/invalid.json || (exit 0)

clean:
	rm -rf $(BUILD)/ rust-project.json
