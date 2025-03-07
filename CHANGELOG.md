# Changelog

All notable changes to this project will be documented in this file.

## [0.2.0] - 2025-03-07

### 🚀 Features

- Add enum support for conditional loading ([1c63a44](https://github.com/sbr075/envoke-rs/commit/1c63a4430e45fd05c919e8d4bb1732e55a7068bc))

### 🐛 Bug Fixes

- Try default on any err for enum ([ca3252c](https://github.com/sbr075/envoke-rs/commit/ca3252c96ad4bc7f7da9146221fa23e038e8f2ec))

### 🎨 Styling

- Fix typos ([f5750a2](https://github.com/sbr075/envoke-rs/commit/f5750a2aae35379cc5eb4cfb20474d0f7515e42a))

### 🧪 Testing

- Add enum tests ([11a93ea](https://github.com/sbr075/envoke-rs/commit/11a93ea8177f09c3d0f29627eb05c30f4b9021b2))

### 🧹 Routine Tasks

- Bump version to 0.2.0 ([9698ba3](https://github.com/sbr075/envoke-rs/commit/9698ba3759ac78560bbf617ddcc1cdb58e635611))
- Bump envoke_derive to 0.2.0 ([2ae5463](https://github.com/sbr075/envoke-rs/commit/2ae5463a55825b95e64b999e6d37c21e54248d71))

## [0.1.7] - 2025-03-03

### 🚀 Features

- Dont panic on macro error ([4c2e4cc](https://github.com/sbr075/envoke-rs/commit/4c2e4cc685ea3aefcc0ae76b6f1495c1f413f104))
- Add custom case for lower-/uppercase ([6bddbce](https://github.com/sbr075/envoke-rs/commit/6bddbce9ec30b27e215ae6ad63acbcc5d7d72cce))

### 🐛 Bug Fixes

- Fix dependency path ([4607ce3](https://github.com/sbr075/envoke-rs/commit/4607ce37ea912807bc994d290c41b539ff0c8552))

### 🧪 Testing

- Removed unnecessary test ([61d2fef](https://github.com/sbr075/envoke-rs/commit/61d2fef53036c239fe1a6f7ebfbc8045749466b6))

### 🧹 Routine Tasks

- Bump version nr up ([4b803c0](https://github.com/sbr075/envoke-rs/commit/4b803c0b9b45c83d7d2cbe54b3b244522c345dcd))

## [0.1.6] - 2025-02-27

### 🐛 Bug Fixes

- Dont return error if env not found for opt ([4e3deac](https://github.com/sbr075/envoke-rs/commit/4e3deacfd55ea299f3a131b9b05dd6a7af930dc3))
- Allow set/map also to be optional ([fb852c9](https://github.com/sbr075/envoke-rs/commit/fb852c94a1ddcd4151ff23499254674fcab88527))
- Dont error if env fails for opts ([81a0459](https://github.com/sbr075/envoke-rs/commit/81a0459e1515260032643522411c382bb7b44adb))

### 🧹 Routine Tasks

- Add examples ([234e045](https://github.com/sbr075/envoke-rs/commit/234e045de5cc9c47067ef727a321b932c978caa0))
- Bump version to 0.1.5 ([5de0a72](https://github.com/sbr075/envoke-rs/commit/5de0a7242e343449a04203cf1b18774da9ea9bc6))
- Bump version to 0.1.6 ([507562c](https://github.com/sbr075/envoke-rs/commit/507562c0e7155bfe73db6a48f788fe3757c7d515))

## [0.1.4] - 2025-02-26

### 🚀 Features

- [**breaking**] Autoset Some() around optional fields ([6b65e81](https://github.com/sbr075/envoke-rs/commit/6b65e81a90dd5f028c573bad64ddbcee77085e5d))
- [**breaking**] Validate field value last ([09b7826](https://github.com/sbr075/envoke-rs/commit/09b7826ec8ccf16d770837e3c4698ec5aad35d75))
- Add option for validating before and after parse ([ed6b817](https://github.com/sbr075/envoke-rs/commit/ed6b8174fd4c8a9f717dcef85f6b4c4ba4347956))
- Dont clone field ident ([03ae45d](https://github.com/sbr075/envoke-rs/commit/03ae45daf2c6d954f6c593d2f5b508bf023cd241))

### 🐛 Bug Fixes

- Use full part for validation error ([208f2c1](https://github.com/sbr075/envoke-rs/commit/208f2c1b167815c10f8d710600d6d70498fe90c3))
- [**breaking**] Dont parse or validate default values ([b9849f8](https://github.com/sbr075/envoke-rs/commit/b9849f860b871b66ae900895619d0d2cfa24a631))
- Switch back to old naming case ([65ebac5](https://github.com/sbr075/envoke-rs/commit/65ebac5247d6bc862b52d12779a6f97d81289b47))

### 📚 Documentation

- Add examples of validation ([4bc0b8d](https://github.com/sbr075/envoke-rs/commit/4bc0b8da1e8d7c57ff8985df3fdd57f4e7c92812))
- Fix licence links ([d495f21](https://github.com/sbr075/envoke-rs/commit/d495f21ded2c374a14f263387ab72e43d6c73035))

### 🧹 Routine Tasks

- Remove unused dependency ([3ef7fff](https://github.com/sbr075/envoke-rs/commit/3ef7fff5133080b90183c8e231b179f78d1e8a1f))
- Bump version to 0.1.4 ([5df5502](https://github.com/sbr075/envoke-rs/commit/5df55024ccd6b9aaf9d6eceb8c444e39d13c9088))

## [0.1.2] - 2025-02-25

### 🚀 Features

- First draft of envoke-rs ([88fdf4f](https://github.com/sbr075/envoke-rs/commit/88fdf4fd8ac9cfa96c0fb0558415ff9ab2ad4e1e))
- Merge default, default_t, and default_fn ([85786de](https://github.com/sbr075/envoke-rs/commit/85786de78dc3809a6d9f3925b66f0c1012b13fbb))
- Allow parse_with for just defaults as well ([c00a9f9](https://github.com/sbr075/envoke-rs/commit/c00a9f94c54cb8692913c567be228903d59d7716))
- Use published derive crate ([1cc8458](https://github.com/sbr075/envoke-rs/commit/1cc8458dbea0e7023fc8fe223ced2a30f0eaa74b))
- Add generic support for set and maps ([c6a151e](https://github.com/sbr075/envoke-rs/commit/c6a151eb3409452fd660a29fa2470a8a4bbd92f8))
- *(envoke)* Make validation error accept any error ([ac039a9](https://github.com/sbr075/envoke-rs/commit/ac039a9d14a9431083d1be83240ac4cf6a092da4))

### 🐛 Bug Fixes

- Fix envoke category slugs ([de394b5](https://github.com/sbr075/envoke-rs/commit/de394b5c6c85c365163eaee2c985a0635e0e224e))
- *(envoke_derive)* Use new validationerror ([0ddf4fe](https://github.com/sbr075/envoke-rs/commit/0ddf4fe21f9d3330308d8068eddc5e4b4a5aebc3))

### 📚 Documentation

- Update examples ([8ef052f](https://github.com/sbr075/envoke-rs/commit/8ef052fa7cbe3be460bdd6a4f2d87947d071fc64))
- Add docs documentation ([9235200](https://github.com/sbr075/envoke-rs/commit/92352000153964c8614984232dcd5949cc9e79ec))
- Add docs documentation ([a4d3192](https://github.com/sbr075/envoke-rs/commit/a4d3192f4b3bc104d9e12c989103d7679404d633))
- Add more information about envoke with example ([f65c497](https://github.com/sbr075/envoke-rs/commit/f65c497081009b212007e7b1771595a9ced6151e))

### 🧹 Routine Tasks

- Bump version to v0.1.1 ([0d2dbc8](https://github.com/sbr075/envoke-rs/commit/0d2dbc8512b2036849ff9d956331c145eebc28b2))

