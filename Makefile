-include .env

sqlite:
	rm -f $(SQLITE_FILE)
	touch $(SQLITE_FILE)
	chmod 775 $(SQLITE_FILE)
