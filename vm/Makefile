TOFU  := $(shell which tofu)
SOURCES := provider.tf backend.tf scaleway.tf variables.tf
VAR_FILE := .scaleway.tfvars
PLAN := plan

$(PLAN): $(SOURCES)
	$(TOFU) plan -out $@ -var-file $(VAR_FILE)

up: $(PLAN)
	$(TOFU) apply $<

down: $(PLAN)
	$(TOFU) destroy -var-file $(VAR_FILE)
	rm $<

.PHONY: up down
