#!/bin/bash
# Transpiles the SCSS into ../styles.css that is used by the build command to
# generate the index.html template

sass styles.scss > ../templates/styles.css
