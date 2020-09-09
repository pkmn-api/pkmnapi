fmt:
	find pkmnapi/src -name '*.rs' -exec rustfmt {} \;
	find pkmnapi/tests -name '*.rs' -exec rustfmt {} \;
	find pkmnapi-db/src -name '*.rs' -exec rustfmt {} \;
	find pkmnapi-db/tests -name '*.rs' -exec rustfmt {} \;
	find pkmnapi-sql/src -name '*.rs' -exec rustfmt {} \;
