const std = @import("std");

const numbers = [_][]const u8{ "one", "two", "three", "four", "five", "six", "seven", "eight", "nine" };

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
            else => @panic("this ain't no digit pal"),
        };
    }

    fn to_char(self: Digit) u8 {
        return @intFromEnum(self) + 48;
    }
};

const allocator = std.heap.page_allocator;

fn isDigit(ch: u8) bool {
    return ch >= 49 and ch <= 57;
}

pub fn main() !void {
    // var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    // defer _ = gpa.deinit();
    // var file = try std.fs.cwd().openFile("input", .{}); // 54951
    var file = try std.fs.cwd().openFile("easy_input_part_one", .{}); // 142
    // var file = try std.fs.cwd().openFile("easy_input_part_two", .{}); // 209, 281
    defer file.close();

    var buf_reader = std.io.bufferedReader(file.reader());
    var in_stream = buf_reader.reader();

    // max line in input file has 51 ASCII characters (+ \n)
    var buf: [52]u8 = undefined;
    var total: u32 = 0;
    while (try in_stream.readUntilDelimiterOrEof(&buf, '\n')) |line| {
        var digits = std.ArrayList(Digit).init(allocator);
        for (line) |char| {
            if (isDigit(char)) {
                try digits.append(Digit.from_char(char));
            }
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
