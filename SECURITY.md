# Sicherheit

- **Keine Geheimnisse ins Repository:** keine Seed-Phrasen, privaten Schlüssel, Keystore-Dateien, API-Keys (z. B. Alchemy) oder Tokens. Die `.gitignore` schließt die üblichen Dateien aus (`local.properties`, `toypay/**/gradle.properties`, `*.jks`, `*.keystore`, `.env` …). Vor jedem Hochladen von App-Code trotzdem prüfen.
- **Kein Mainnet:** Alles in diesem Repository ist Testnet-Prototyp. Keine echten Schlüssel, keine echten Gelder.
- **Schwachstellen melden:** bitte nicht öffentlich, sondern direkt an den Projektinhaber. Ein Bug-Bounty-Programm ist vorgesehen (aus der Reserve, siehe `docs/01-spezifikation.md`, Abschnitt 10).
