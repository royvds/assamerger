# ASSA Merger

> An ASS / SSA subtitle merger

This tool is intended to merge the translations from one subtitle into the timings of another, ignoring non-dialogue lines such as signs & songs. An example is to merge the translations from one fansub group with the timings (and remaining non-dialogue lines) of another group.

The program first matches subtitle lines by iterating through all lines and detecting lines that have to be removed from the base subtitle file (the one with the correct timings) and lines that have to be copied over from the dialogue subtitle file (the one with the better dialogue/translations). Then the timings will be copied over.

This idea has been originally tested in python, see [assDialogueMerger](https://github.com/royvds/assDialogueMerger). This new version in Rust extends upon this proof-of-concept with semantic similarity and a GUI to enable easy manual adjustments to the alignment of subtitle lines.


### Project Status

This tool is currently not operational. Development is focussed on refactoring old experimenting/testing code and developing a proper event alignment system that uses both levenshtein and semantic similarity.


### Additional info

Please refer to the [libass repository](https://github.com/libass/libass) for details on the ASS / SSA subtitle standard.


### Credits

This program uses the [all-MiniLM-L6-v2](https://huggingface.co/sentence-transformers/all-MiniLM-L6-v2) language model to capture semantic information of subtitle text. 

