# WorldKeeper - Game Design Document

## Table of Contents
1. [Game Overview](#game-overview)
2. [Core Mechanics](#core-mechanics)
3. [Game Systems](#game-systems)
4. [Technical Architecture](#technical-architecture)
5. [Asset Requirements](#asset-requirements)
6. [Development Roadmap](#development-roadmap)

## Game Overview

### Vision Statement
To create a versatile and extendable game where the player grows in power through each iteration of a playthrough. Making it fun and engaging.

### Genre
GodSim

### Target Platform
- Primary: Desktop (Windows, macOS, Linux)
- Secondary: Web (WASM)

### Core Pillars
*List 3-5 fundamental design principles that guide all decisions*

1. **Roguelike mechanism**: When the worldkeepr dies, the game is over.
2. **Ascension**: During a game the worldkeeper gains ascension points which will increase the power of a new game.
3. **Flexibility**: The options should be vast and useful.

## Core Mechanics

### Primary Mechanics
The World Keeper game is a game where the player plays a keeper of worlds. This means that the player needs to create a world and then tend to its needs. The player cannot directly influence the world unless he creates an avatar of himself which then also subjects him to permanent death.
The player has to fend off attempt of other worldkeepers who want to take over other worlds and likewise, the player can opt to take over worlds of others.
The player can influence the following:
* Type of world (climate, what it is made up of, moons and other generics)
* Tyoe of physics that are used (Standard Einstein universe, Fantasy realm, High Fantasy, Science-Fantasy, Science-Fiction, Superhero, etc)
* The type of races that will live on the planet
* There will be a point system to pick out special abilities for the planet/races/worldkeeper to pick from.

#### Mechanic 1: [Name]
- **Description**: What the player does
- **Input**: How the player interacts
- **Feedback**: What the player sees/hears
- **Purpose**: Why this mechanic exists

#### Mechanic 2: [Name]
- **Description**: 
- **Input**: 
- **Feedback**: 
- **Purpose**: 

### Secondary Mechanics
*Supporting mechanics that enhance the core experience*

## Game Systems

### World System
- **World Generation**: How the game world is created
- **Environment**: Static and dynamic elements
- **Physics**: Movement, collision, etc.

### Player System
- **Character**: Player representation
- **Controls**: Input mapping and responsiveness
- **Progression**: How the player character develops

### UI System
- **Menus**: Navigation and settings
- **HUD**: In-game information display
- **Inventory**: Item management (if applicable)

### Audio System
- **Music**: Background and dynamic music
- **SFX**: Sound effects for actions and events
- **Voice**: Dialog and narration (if applicable)

## Technical Architecture

### Bevy Systems Organization

#### Core Systems
- Game state management
- Input handling
- Camera control
- Scene management

#### Game-Specific Systems
*List your game's unique systems*

#### System Dependencies
```
Update Order:
1. Input System
2. Game Logic Systems
3. Physics System
4. Rendering System
```

### Entity Component System (ECS) Design

#### Components
| Component | Purpose | Data |
|-----------|---------|------|
| Transform | Position, rotation, scale | Vec3, Quat, Vec3 |
| Player | Mark player entity | Player ID |
| Health | Entity health | current, max |
| *Add your components* | | |

#### Resources
| Resource | Purpose | Data |
|----------|---------|------|
| GameState | Current game state | enum variant |
| Time | Game timing | delta, elapsed |
| *Add your resources* | | |

## Asset Requirements

### Graphics
- **Sprites**: Character, environment, UI elements
- **Textures**: Materials and surfaces
- **Animations**: Character movements, effects

### Audio
- **Music**: Background tracks for different scenes
- **SFX**: Action sounds, ambient effects
- **Voice**: Character dialog (if applicable)

### Fonts
- UI fonts for menus and HUD
- Special fonts for styling

## Development Roadmap

### Phase 1: Foundation (Weeks 1-2)
- [ ] Basic Bevy setup and window creation
- [ ] Input handling system
- [ ] Basic player entity and movement
- [ ] Camera system

### Phase 2: Core Gameplay (Weeks 3-4)
- [ ] Implement core mechanics
- [ ] Basic UI system
- [ ] Asset loading system
- [ ] Game state management

### Phase 3: Content & Polish (Weeks 5-6)
- [ ] Add game content
- [ ] Audio integration
- [ ] Visual effects
- [ ] Performance optimization

### Phase 4: Testing & Release (Weeks 7-8)
- [ ] Playtesting and bug fixes
- [ ] Balance adjustments
- [ ] Documentation and packaging
- [ ] Release preparation

## Ideas and Brainstorming

### Feature Ideas
*Use this section to capture ideas that might be implemented later*

- Feature idea 1
- Feature idea 2
- Feature idea 3

### Technical Challenges
*Identify potential technical hurdles early*

- Challenge 1: Description and potential solutions
- Challenge 2: Description and potential solutions

### Questions to Resolve
*Track open questions that need answers*

- [ ] Question 1
- [ ] Question 2
- [ ] Question 3

---

*This document is a living document and should be updated throughout development*