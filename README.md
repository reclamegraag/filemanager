# File Manager

Een snelle, moderne dual-pane bestandsbeheerder gebouwd met Tauri, SvelteKit en TypeScript.

---

## Gebruikershandleiding

### Overzicht

File Manager is een krachtige bestandsbeheerder met twee panelen, geïnspireerd door klassieke tools zoals Norton Commander en Total Commander. Navigeer efficiënt door je bestanden met het toetsenbord of de muis.

### Interface

```
┌─────────────────────────────────────────────────────────────┐
│  Zijbalk          │  Linker Paneel    │  Rechter Paneel    │
│  ───────────────  │  ──────────────── │  ──────────────────│
│  📁 Bladwijzers   │  📂 Documents     │  📂 Downloads      │
│  📁 WSL Distros   │  📄 rapport.pdf   │  🖼️ foto.jpg       │
│  🕐 Recent        │  📁 project       │  📦 backup.zip     │
│                   │                   │                    │
└─────────────────────────────────────────────────────────────┘
│  Statusbalk: /home/user/Documents  •  3 items geselecteerd │
└─────────────────────────────────────────────────────────────┘
```

### Sneltoetsen

#### Navigatie

| Toets | Actie |
|-------|-------|
| `↑` `↓` | Selectie omhoog/omlaag |
| `Enter` | Map openen of bestand uitvoeren |
| `Backspace` | Naar bovenliggende map |
| `Tab` | Wisselen tussen panelen |
| `Home` / `End` | Naar eerste/laatste item |
| `Ctrl+1` t/m `Ctrl+9` | Spring naar bladwijzer |

#### Selectie

| Toets | Actie |
|-------|-------|
| `Spatie` | Item selecteren/deselecteren |
| `Shift+↑/↓` | Selectie uitbreiden |
| `Ctrl+A` | Alles selecteren |
| Klik | Enkel item selecteren |
| `Ctrl+Klik` | Item toevoegen aan selectie |
| `Shift+Klik` | Bereik selecteren |

#### Bestandsoperaties

| Toets | Actie |
|-------|-------|
| `F2` | Hernoemen |
| `F5` | Kopiëren naar ander paneel |
| `F6` | Verplaatsen naar ander paneel |
| `F7` | Nieuwe map aanmaken |
| `F8` of `Delete` | Verwijderen |
| `Ctrl+Z` | Ongedaan maken |

#### Interface

| Toets | Actie |
|-------|-------|
| `Ctrl+P` | Commandopalet openen |
| `/` | Zoeken/filteren |
| `Escape` | Filter wissen |
| `F1` | Help weergeven |

### Functies

#### Dual-Pane Weergave
Werk met twee mappen tegelijk. Ideaal voor het kopiëren of verplaatsen van bestanden tussen locaties.

#### Bladwijzers
Sla veelgebruikte mappen op als bladwijzer. Toegankelijk via de zijbalk of met `Ctrl+1` t/m `Ctrl+9`.

#### WSL Integratie
Directe toegang tot je Windows Subsystem for Linux distributies vanuit de zijbalk.

#### Batch Hernoemen
Selecteer meerdere bestanden en hernoem ze met patronen:
- `{name}` — originele bestandsnaam
- `{ext}` — extensie
- `{n}` — volgnummer
- `{date}` — huidige datum

*Voorbeeld:* `foto_{n}.{ext}` hernoemt naar `foto_1.jpg`, `foto_2.jpg`, etc.

#### Sorteren
Klik op kolomkoppen om te sorteren op:
- Naam
- Extensie
- Grootte
- Gewijzigd

#### Commandopalet
Druk op `Ctrl+P` voor snelle toegang tot alle commando's. Typ om te zoeken.

### Tips

- **Snel navigeren:** Typ `/` en begin te typen om direct te filteren op bestandsnaam.
- **Efficiënt kopiëren:** Navigeer in het ene paneel naar de bron, in het andere naar de bestemming, selecteer bestanden en druk `F5`.
- **Verborgen bestanden:** Toggle zichtbaarheid via het commandopalet.

---

## Development

### Recommended IDE Setup

[VS Code](https://code.visualstudio.com/) + [Svelte](https://marketplace.visualstudio.com/items?itemName=svelte.svelte-vscode) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer).

### Tech Stack

- **Frontend:** SvelteKit + TypeScript
- **Backend:** Tauri (Rust)
- **Styling:** Tailwind CSS
