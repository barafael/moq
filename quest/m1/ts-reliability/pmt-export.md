# [S] Synced TS legs agree on catalog-only revisions

## Goal

Under `moq export ts --sync`, a catalog change no stream's media anchors
lands at the same slot on every leg, including one that joins after it, or
is refused loudly. That covers a `programDescriptors` change and a
`muxRate` change. Today each leg logs and ignores them, so a leg that opens
after the change lays different tables or a different grid than the legs
that saw it.

## Plan

A track added, a track that ends, and a PID another stream takes over
already revise the PMT and the lanes at the first table repetition two
slots past the jitter-buffer key of that stream's first or last unit, which
every leg reads alike. The revised `version_number` follows from what the
PMT lists, so a joiner announces it alike. The catalog carries no media
time, so nothing in it says where a catalog-only change would land.

Options, to settle with the maintainer:

- The TS importer records where the change took effect, as a media time
  (and perhaps the source's PMT `version_number`) in the `mpegts` section.
  Export lands it there. A wire change, so the MPEG-TS draft changes too.
- The change rides the next media-anchored revision, as one transaction. A
  leg that opens in between differs until then.
- Keep ignoring it, and document that a leg must restart to take it up.

A `muxRate` change also needs lane positions that survive a rate change:
today every layout counts packets at one rate.

Also open: a track without a recorded PID takes the next free one in catalog
name order, so a track added under a name that sorts before the others gets
a different PID on a leg that joins later. TS sources record their PIDs.

## Related

- [PAT changes](/quest/m1/ts-reliability/pat-lifecycle.md) - a moved PMT PID reaches the export as a new PAT
