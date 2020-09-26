fmt:
	find src -name '*.rs' -exec rustfmt {} \;
	find pkmnapi-api/src -name '*.rs' -exec rustfmt {} \;
	find pkmnapi-api/tests -name '*.rs' -exec rustfmt {} \;
	find pkmnapi-db/src -name '*.rs' -exec rustfmt {} \;
	find pkmnapi-db/tests -name '*.rs' -exec rustfmt {} \;
	find pkmnapi-email/src -name '*.rs' -exec rustfmt {} \;
	find pkmnapi-sql/src -name '*.rs' -exec rustfmt {} \;
