# Preview capture

The repository keeps one representative, deterministic screenshot for each
template under `assets/previews/`. These images are tracked because they give
visitors an immediate preview before cloning the repository.

## Regenerate the README gallery

From the repository root, run:

```bash
./scripts/capture-readme-previews.sh
```

The script uses Slint's software renderer, a scale factor of `1`, and a
1280×800 viewport. It shares one ignored Cargo target directory so the four
applications do not rebuild the complete dependency graph independently.

The captured states are:

| Template | State |
|---|---|
| Command | Overview |
| Forge | Explorer |
| Fleet | Overview |
| Ledger | Portfolio |

Review all four images before committing them. Text rendering can vary across
operating systems and font installations even when the renderer and viewport
are fixed.

## Animated previews

An animated GIF is possible, but static PNGs are the default because they load
faster, remain sharper, and produce reviewable diffs. Use animation only when
it demonstrates interaction that a still image cannot explain, such as opening
a drawer, changing a chart period, or collapsing an editor panel.

For an animated preview:

1. Capture a short interaction at the same viewport and scale factor.
2. Crop it to the application window.
3. Keep the duration below roughly 8–12 seconds.
4. Export at 960–1280 pixels wide and optimize the palette.
5. Keep the resulting file small enough for a fast README load, preferably
   below 5 MB.

Store published animations in `assets/previews/`. Temporary frames and visual
comparison output belong in the ignored `ai/` directory.
