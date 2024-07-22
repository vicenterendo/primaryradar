# 📡 PrimaryRadar Plugin
The PrimaryRadar Euroscope plugin provides simulation for a primary radar, taking into account terrain and radar range and other properties.

![README-02](https://github.com/user-attachments/assets/a89f2f38-0b67-416a-99e2-9df8b647d030)

## 📄 Terrain Data
Terrain data should be converted from the Geotiff format to the format described on the above image using the `/converter` tool.
The program creates a `/geodata` folder on the working directory with a `geo.dat` and a `meta.json` file inside:
- `geo.dat`: Vec<u8> that contains all rows of the geotiff grid joined together in one single vector.
- `meta.json`: contains the data necessary to translate real-life coordinates into individual pixels on the grid, as well as it's size.
