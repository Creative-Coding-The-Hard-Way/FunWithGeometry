initSidebarItems({"enum":[["CompressedBlock","The raw, possibly compressed pixel data of a file. Each layer in a file can have a different type. Also contains positioning information that locates this data block in the corresponding layer. Exists inside a `Chunk`."]],"struct":[["Chunk","A generic block of pixel information. Contains pixel data and an index to the corresponding header. All pixel data in a file is split into a list of chunks. Also contains positioning information that locates this data block in the referenced layer."],["CompressedDeepScanLineBlock","This `Block` consists of one or more deep scan lines. Corresponds to type attribute `deepscanline`."],["CompressedDeepTileBlock","This `Block` is a tile of deep data. Corresponds to type attribute `deeptile`."],["CompressedScanLineBlock","A `Block` of possibly compressed flat scan lines. Corresponds to type attribute `scanlineimage`."],["CompressedTileBlock","This `Block` is a tile of flat (non-deep) data. Corresponds to type attribute `tiledimage`."],["TileCoordinates","Indicates the position and resolution level of a `TileBlock` or `DeepTileBlock`."]]});