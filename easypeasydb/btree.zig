const std = @import("std");

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    const allocator = gpa.allocator();

    var node = try BNode.init(allocator);
    defer node.deinit();

    try node.keys.append(10);
    try node.keys.append(20);
    try node.keys.append(30);
    try node.keys.append(40);

    node.display();
}

pub const BNode = struct {
    allocator: std.mem.Allocator,
    parent: ?*BNode,
    children: std.ArrayList(*BNode),
    keys: std.ArrayList(i32),

    pub fn init(allocator: std.mem.Allocator) !BNode {
        return BNode{
            .allocator = allocator,
            .parent = null,
            .children = std.ArrayList(*BNode).init(allocator),
            .keys = std.ArrayList(i32).init(allocator),
        };
    }

    pub fn display(self: *const BNode) void {
        std.debug.print("BNode: [", .{});
        for (self.keys.items) |key| {
            std.debug.print("{}, ", .{key});
        }
        std.debug.print("]\n", .{});
    }

    pub fn deinit(self: *const BNode) void {
        self.children.deinit();
        self.keys.deinit();
    }
};
