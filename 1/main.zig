const std = @import("std");

const Digit = enum(u8) {
    one = 1,
    two,
    three,
    four,
    five,
    six,
    seven,
    eight,
    nine,
    fn from_char(ch: u8) Digit {
        return switch (ch - 48) {
            1 => Digit.one,
            2 => Digit.two,
            3 => Digit.three,
            4 => Digit.four,
            5 => Digit.five,
            6 => Digit.six,
            7 => Digit.seven,
            8 => Digit.eight,
            9 => Digit.nine,
            else => unreachable,
        };
    }

    fn to_char(self: Digit) u8 {
        return @intFromEnum(self) + 48;
    }

    fn from_written(s: []const u8) ?Digit {
        if (std.mem.startsWith(u8, s, "one")) {
            return Digit.one;
        }
        if (std.mem.startsWith(u8, s, "two")) {
            return Digit.two;
        }
        if (std.mem.startsWith(u8, s, "three")) {
            return Digit.three;
        }
        if (std.mem.startsWith(u8, s, "four")) {
            return Digit.four;
        }
        if (std.mem.startsWith(u8, s, "five")) {
            return Digit.five;
        }
        if (std.mem.startsWith(u8, s, "six")) {
            return Digit.six;
        }
        if (std.mem.startsWith(u8, s, "seven")) {
            return Digit.seven;
        }
        if (std.mem.startsWith(u8, s, "eight")) {
            return Digit.eight;
        }
        if (std.mem.startsWith(u8, s, "nine")) {
            return Digit.nine;
        }
        return null;
    }
};

const allocator = std.heap.page_allocator;

fn isDigit(ch: u8) bool {
    return ch >= 49 and ch <= 57;
}

pub fn main() !void {
    var file = try std.fs.cwd().openFile("input", .{}); // 54951, 55218
    // var file = try std.fs.cwd().openFile("easy_input_part_one", .{}); // 142
    // var file = try std.fs.cwd().openFile("easy_input_part_two", .{}); // 209, 281
    defer file.close();

    var buf_reader = std.io.bufferedReader(file.reader());
    var in_stream = buf_reader.reader();

    // max line in input file has 51 ASCII characters (+ \n)
    var buf: [52]u8 = undefined;
    var total: u32 = 0;
    while (try in_stream.readUntilDelimiterOrEof(&buf, '\n')) |line| {
        var digits = std.ArrayList(Digit).init(allocator);
        var i: u16 = 0;
        for (line) |char| {
            if (isDigit(char)) {
                try digits.append(Digit.from_char(char));
            } else if (Digit.from_written(line[i..])) |digit| {
                try digits.append(digit);
            }
            i += 1;
        }

        if (digits.items.len == 1) {
            try digits.append(digits.items[0]);
        }

        var s = [_]u8{ 'h', 'e' };
        s[0] = digits.items[0].to_char();
        s[1] = digits.items[digits.items.len - 1].to_char();

        // std.debug.print("n: {d}\n", .{n});

        total += try std.fmt.parseInt(u8, &s, 10);
    }
    std.debug.print("{d}\n", .{total});
}
