# Cats Destroyer 2000

[![Bevy tracking](https://img.shields.io/badge/Bevy%20tracking-0.14-lightblue)](https://github.com/bevyengine/bevy/blob/main/docs/plugins_guidelines.md#main-branch-tracking)
[![MIT/Apache 2.0](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](https://github.com/fabinistere/bevy_turn-based_combat#license)

CreaJeu Edition 2023 - Theme: ***One Object Many Use***

![fast_blue_cat](https://user-images.githubusercontent.com/73140258/216720606-6e8f7768-3170-4956-a5d1-5124741783aa.gif)

<https://github.com/user-attachments/assets/3b8384c0-938f-400f-a038-8838310e54cb>

![tablet-preview](https://github.com/user-attachments/assets/dec29ae4-a369-43e5-a7d3-84c1f283a48e)
![mind-control-preview](https://github.com/user-attachments/assets/04081e3f-f08c-4168-8965-91c118b96311)

## Assets are excluded from git storage

Due to the inefficiency to store image in git,
all asset can be download here (from the latest version):
[Latest Assets - Google Drive](https://drive.google.com/drive/folders/1qk5_bIUzAUFTuI2A_C0CLmAFGyPDFsMR?usp=share_link)

or in the correct release note (if from other version):
[Releases - Github](https://github.com/Wabtey/cats_destroyer_2000/releases)

## Rules

***Goal***: Flee from the lab!

### Binding

- `M` or `:` to mindcontrol a cat
  - `ESC` to return to the player body
- `ESC` to open the tablet
  - *click* on the side door to hack doors
- `Z Q S D` or `W A S D` or `Up Left Down Right` to move

### Future

- [x] LevelScene, CinematicScene
  - [x] Reset the scene when touched (by an enemy)
- [x] Final Cinematic
  - [x] Animations
  - [x] Black Cat, EasterEgg
- [x] Hacking tablet
- [ ] Limited Vision
  - [ ] Hack camera
    - [ ] Mind control through camera's needle shots
- [ ] TODO: polish - Background and MovingBubbles

### Level Design

#### Tutorial

A long corridor, with compartments to the left and right.
A normal cat is walking down the corridor.
The exit is at the end of the corridor, blocked by the kitten.
The player must

- access a terminal in order to open one of the doors on the sides (maybe just one door for clarity)
- mind control the kitten to move it into the cupboard
- Press the global red button with the mind controlled cat
- rehack before the kitten's daze wears off
  - having a delay after mind control can allow you to chain actions like this
- slide to exit

### Mechanics

#### Tablet

Through the tablet overlay

- Hack doors;
- Take control of the cameras available;
  - Uncover some rooms with the camera's vision;
  - The cameras will allow you to shot needles of mind control to the scientists;
- Sabotage electronics to distract attention;
- Press `esc` to exit the tablet and re-control your character.

Little by little, the level and the entities inside it are revealed.

<!-- ### Idées

vv———————————————————————————————————————vv

Principe du gameplay :
Utiliser la tablette de contrôle pour contrôler les autres chats et les objets → One Object Many Use
Tablette servira aussi d’inventaire et d’interface.

Différents types de chats qui représentent différents ennemis ?

Types de chats :

- Chat Normal → Peut juste être déplacé
- Chat Savant → Peut hacker des ordis/ouvrir des portes
- Chat Espion → A une vision sous forme de cône ou bien de ligne droite (genre laser qui si détecte fonce sur le joueur)
- Chat Costaud → Peut casser différents objets

Nombre de croquettes max qui peut être reset que si on reset le level
(ou le même style que la gestion des barrettes de RAM dans cyberpunk)

Des collectibles style des clés pour déverrouiller des portes -->
