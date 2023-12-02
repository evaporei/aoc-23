const std = @import("std");

const numbers = [_][]const u8{ "one", "two", "three", "four", "five", "six", "seven", "eight", "nine" };

const allocator = std.heap.page_allocator;

pub fn main() !void {
    // var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    // defer _ = gpa.deinit();
    // var file = try std.fs.cwd().openFile("input", .{}); // 54951
    // var file = try std.fs.cwd().openFile("easy_input_part_one", .{}); // 142
    var file = try std.fs.cwd().openFile("easy_input_part_two", .{}); // 209, 281
    defer file.close();

    var buf_reader = std.io.bufferedReader(file.reader());
    var in_stream = buf_reader.reader();

    // max line in input file has 51 ASCII characters (+ \n)
    var buf: [52]u8 = undefined;
    var total: u32 = 0;
    while (try in_stream.readUntilDelimiterOrEof(&buf, '\n')) |line| {
        var n: u8 = 0;
        var first = true;
        var last: u8 = 0;
        var currWord = std.ArrayList(u8).init(allocator);
        defer currWord.deinit();
        var str_n = std.ArrayList(u8).init(allocator);
        defer str_n.deinit();
        for (line) |char| {
            if (char >= 49 and char <= 57) {
                if (currWord.items.len != 0) {
                    var i: u8 = 0;
                    for (numbers) |numText| {
                        if (std.mem.containsAtLeast(u8, currWord.items, 1, numText)) {
                            try str_n.append(i + 1 + 48);
                        }
                        i += 1;
                    }
                }
                std.debug.print("word before n: {s}\n", .{currWord.items});
                try currWord.resize(0);
                // for (currWord.items) |_| {
                //     _ = currWord.pop();
                // }
                // currWord = "";
                if (first) {
                    n += (char - 48) * 10;
                    first = false;
                }

                try str_n.append(char);
                last = char - 48;
            } else {
                // _ = try std.fmt.bufPrint(currWord, "{c}", .{char});
                // currWord = try currWord.append(char);
                try currWord.append(char);
            }
        }
        if (currWord.items.len != 0) {
            var i: u8 = 0;
            for (numbers) |numText| {
                if (std.mem.containsAtLeast(u8, currWord.items, 1, numText)) {
                    try str_n.append(i + 1 + 48);
                }
                i += 1;
            }
        }
        // var i = try std.fmt.parseInt(u8, currWord.items, 10);
        std.debug.print("word: {s}\n", .{currWord.items});
        std.debug.print("str_n: {s}\n", .{str_n.items});
        n += last;
        std.debug.print("n: {d}\n", .{n});
        total += n;
    }
    std.debug.print("{d}\n", .{total});
}
