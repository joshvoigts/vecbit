serve:
	cargo watch -i web/static -i db.sqlite -x run # & zola --root web serve && kill $$!
