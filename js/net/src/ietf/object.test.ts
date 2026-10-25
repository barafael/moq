import { expect, test } from "bun:test";
import { Reader, Writer } from "../stream.ts";
import { FetchHeader } from "./fetch.ts";
import { Group } from "./object.ts";
import { Version } from "./version.ts";

test("a data stream's header goes out in one write, stream type included", async () => {
	for (const version of [Version.DRAFT_14, Version.DRAFT_22]) {
		const chunks: Uint8Array[] = [];
		const writer = new Writer(
			new WritableStream<Uint8Array>({ write: (chunk) => void chunks.push(new Uint8Array(chunk)) }),
			version,
		);

		// Every optional field present, with values past a one-byte varint.
		const group = new Group({
			trackAlias: 1_000_000n,
			groupId: 70_000,
			subGroupId: 300,
			publisherPriority: 200,
			flags: {
				hasExtensions: true,
				hasSubgroup: true,
				hasSubgroupObject: false,
				hasEnd: true,
				hasPriority: true,
				firstObject: true,
			},
		});
		await group.encode(writer, version);
		await new FetchHeader({ requestId: 1_000_000n }).encode(writer, version);
		expect(chunks.length).toBe(2);

		const decoded = await Group.decode(new Reader(undefined, chunks[0], version), version);
		expect(decoded).toEqual(group);

		const fetch = new Reader(undefined, chunks[1], version);
		expect(await fetch.u53()).toBe(FetchHeader.type);
		expect((await FetchHeader.decode(fetch, version)).requestId).toBe(1_000_000n);
		expect(await fetch.done()).toBe(true);
	}
});
