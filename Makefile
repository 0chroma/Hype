install-gsettings:
	install -D data/{{app-id}}.gschema.xml /usr/share/glib-2.0/schemas/{{app-id}}.gschema.xml
	glib-compile-schemas /usr/share/glib-2.0/schemas

uninstall-gsettings:
	rm /usr/share/glib-2.0/schemas/{{app-id}}.gschema.xml
	glib-compile-schemas /usr/share/glib-2.0/schemas

update-flatpak-deps:
	wget 'https://github.com/flatpak/flatpak-builder-tools/archive/refs/heads/master.zip'
	unzip -o master.zip -d _flatpak-builder-tools
	cd _flatpak-builder-tools/flatpak-builder-tools-master/cargo/; \
	poetry env activate; \
	poetry install; \
	poetry run ./flatpak-cargo-generator.py ../../../Cargo.lock -o ../../../build-aux/cargo-sources.json
	rm -Rf _flatpak-builder-tools
	rm master.zip
