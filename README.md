# **RustyBuddy**

🎉 **RustyBuddy** is a fun and interactive CLI-based pet simulation game written in Rust! Create your virtual pet, take care of it, and watch it grow. Feed, play, and let your buddy sleep while keeping track of its stats and mood.

## **Features**

- 🐾 **Create Your Buddy**: Choose a name and type (e.g., Cat, Dog, Dragon) for your virtual pet.
- 🎭 **Dynamic Moods**: Your pet's mood changes based on hunger, happiness, and energy.
- 🍖 **Feed Your Pet**: Satisfy your pet’s hunger with food.
- 🎾 **Play With Your Pet**: Increase your pet's happiness by playing games.
- 💤 **Let Your Pet Sleep**: Restore your pet's energy with rest.
- 🕒 **Aging System**: Your pet grows older over time.
- 🎨 **ASCII Animations**: Cute animations for actions like eating, playing, and sleeping.
- 🏆 **Achievements**: Celebrate milestones like birthdays and achievements.
- 💾 **Save and Load**: Save your pet’s state and continue later.

---

## **Installation**

### Prerequisites
- Install [Rust](https://www.rust-lang.org/tools/install) if you haven't already.

### Install via `cargo`
```bash
cargo install rustybuddy
```

---

## **Usage**

Run the program from your terminal using the `rustybuddy` command.

### **Commands**

| Command                         | Description                          |
|---------------------------------|--------------------------------------|
| `rustybuddy new --name <NAME> --kind <KIND>` | Create a new pet with a name and type (e.g., `--name Tommy --kind Cat`). |
| `rustybuddy stats`              | View your pet's stats (hunger, happiness, energy, health, age, and mood). |
| `rustybuddy feed`               | Feed your pet to reduce hunger.      |
| `rustybuddy play`               | Play with your pet to increase happiness. |
| `rustybuddy sleep`              | Let your pet sleep to restore energy. |
| `rustybuddy save`               | Save your pet's current state.       |
| `rustybuddy load`               | Load a previously saved pet.         |

---

## **Examples**

### Create a New Pet
```bash
rustybuddy new --name Fluffy --kind Dragon
```

### View Stats
```bash
rustybuddy stats
```

### Feed Your Pet
```bash
rustybuddy feed
```

### Play with Your Pet
```bash
rustybuddy play
```

### Save and Load
Save the pet state:
```bash
rustybuddy save
```
Load a saved pet:
```bash
rustybuddy load
```

---

## **How It Works**
RustyBuddy uses a JSON file (`pet.json`) to save your pet's state. The pet's stats (hunger, happiness, energy, health, and age) decay over time, so regular care is necessary to keep your buddy happy and healthy.

---

## **Contributing**

Contributions are welcome! If you’d like to improve the game, follow these steps:
1. Fork the repository.
2. Create a new branch:
   ```bash
   git checkout -b feature/awesome-feature
   ```
3. Commit your changes:
   ```bash
   git commit -m "Add awesome feature"
   ```
4. Push your branch:
   ```bash
   git push origin feature/awesome-feature
   ```
5. Open a pull request.

---

## **License**

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

---

## **Future Enhancements**

- 🐉 **Pet Evolution**: Unlock new appearances and abilities as your pet grows.
- 🌎 **Multiplayer Mode**: Let pets interact with each other.
- 📱 **Mobile Version**: Build a mobile app for RustyBuddy.
- 🎮 **Mini-Games**: Add fun mini-games to play with your pet.

---

## **Author**

Created with ❤️ by [Krish Maity](https://github.com/ktshacx).

---
