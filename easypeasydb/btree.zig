const std = @import("std");

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    const allocator = gpa.allocator();

    var node = try BNode.init(allocator);
    defer node.deinit();

    try node.insert(10);
    try node.insert(20);
    try node.insert(30);

    node.display();
}

pub fn reverse(comptime T: type, arrayList: *std.ArrayList(T)) void {
    var i: usize = 0;
    var j: usize = arrayList.items.len - 1;
    while (i < j) {
        const temp = arrayList.items[i];
        arrayList.items[i] = arrayList.items[j];
        arrayList.items[j] = temp;
        i += 1;
        j -= 1;
    }
}

pub const BTREE_DEGREE = 4;

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

    pub fn insert(self: *BNode, key: i32) !void {
        if (self.children.items.len == 0) {
            try self.keys.append(key);
            std.mem.sort(i32, self.keys.items, {}, std.sort.asc(i32));
            if (self.keys.items.len == BTREE_DEGREE) {
                try self.split();
            }
            return;
        }
        var child: usize = 0;
        for (self.keys) |item| {
            if (item > key) {
                break;
            }
            child += 1;
        }
        self.children[child].insert(key);
    }

    pub fn split(self: *BNode) !void {
        var split_node: *BNode = try self.allocator.create(BNode);
        const mid = BTREE_DEGREE / 2;
        var cur: usize = mid + 1;
        while (cur < self.keys.items.len) : (cur += 1) {
            try split_node.keys.append(self.keys.pop());
            if (self.children.items.len > 0) {
                try split_node.children.append(self.children.pop());
            }
        }
        reverse(i32, &split_node.keys);
        if (self.children.items.len > 0) {
            try split_node.children.append(self.children.pop());
            reverse(*BNode, &split_node.children);
        }
        const to_root = self.keys.pop();

        if (self.parent == null) {
            const parent: *BNode = try self.allocator.create(BNode);
            self.parent = parent;
        }
        split_node.parent = self.parent;
        try self.parent.?.force_insert(to_root, self, split_node);
    }

    fn force_insert(self: *BNode, key: i32, left: *BNode, right: *BNode) !void {
        if (self.keys.items.len == 0) {
            try self.keys.append(key);
            try self.children.append(left);
            try self.children.append(right);
            return;
        }
        var insert_pos: usize = 0;
        for (self.keys.items, 0..) |item, i| {
            if (item > key) {
                break;
            }
            insert_pos = i;
        }
        try self.keys.append(key);
        try self.children.append(right);
        var cur: usize = self.keys.items.len - 1;
        while (cur > insert_pos) : (cur -= 1) {
            const temp = self.keys.items[cur];
            self.keys.items[cur] = self.keys.items[cur - 1];
            self.keys.items[cur - 1] = temp;
        }
        if (self.keys.items.len == BTREE_DEGREE) {
            self.split() catch unreachable;
        }
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
