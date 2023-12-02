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

    fn fromChar(ch: u8) Digit {
        return @enumFromInt(ch - 48);
    }

    fn toChar(self: Digit) u8 {
        return @intFromEnum(self) + 48;
    }

    fn fromWritten(str: []const u8) ?Digit {
        inline for (@typeInfo(Digit).Enum.fields) |field| {
            if (std.mem.startsWith(u8, str, field.name))
                return @enumFromInt(field.value);
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
                try digits.append(Digit.fromChar(char));
            } else if (Digit.fromWritten(line[i..])) |digit| {
                try digits.append(digit);
            }
            i += 1;
        }

        if (digits.items.len == 1) {
            try digits.append(digits.items[0]);
        }

        var s = [_]u8{ 'h', 'e' };
        s[0] = digits.items[0].toChar();
        s[1] = digits.items[digits.items.len - 1].toChar();

        // std.debug.print("n: {d}\n", .{n});

        total += try std.fmt.parseInt(u8, &s, 10);
    }
    std.debug.print("{d}\n", .{total});
}
