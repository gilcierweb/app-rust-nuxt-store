<template>
  <div class="mb-6 justify-between flex items-center ">
        <form action="">

            <input type="text" placeholder="Product Name" class="input input-bordered w-full mb-4" />

            <div class="grid grid-cols-1 gap-6 md:grid-cols-2">

                <input type="number" placeholder="Price" class="input input-bordered w-full mb-4" />
            </div>

            <textarea placeholder="Description" class="textarea textarea-bordered w-full mb-4"></textarea>

            <h2>Product Image</h2>

            <input type="file" name="image" id="image" class="file-input file-input-bordered w-full mb-4" />

            <h2>Product Variants</h2>
            <div class="grid grid-cols-1 gap-6 md:grid-cols-2">
                <input type="text" placeholder="Variant Name" class="input input-bordered w-full mb-4" />
                <input type="text" placeholder="Variant Value" class="input input-bordered w-full" />
                <button class="btn btn-primary" data-repeater-create="">
                    <i class="icon-base bx bx-plus icon-sm me-2"></i>
                    Add another option
                </button>
            </div>

            <h2>Product Inventory</h2>

            <h2>Product Pricing</h2>

            <!-- <div class="card w-full">
                <div class="card-body"> -->
                    <h2>Product Organize</h2>
                    Vendor
                    <input type="text" placeholder="Vendor" class="input input-bordered w-full mb-4" />

                    Category
                    <input type="text" placeholder="Category" class="input input-bordered w-full mb-4" />
                    Collection
                    <input type="text" name="Collection" id="Collection" placeholder="Collection"
                        class="input input-bordered w-full mb-4" />
                    Status
                    <input type="text" name="Status" id="Status" placeholder="Status"
                        class="input input-bordered w-full mb-4" />
                    Tags
                    <input type="text" name="Tags" id="Tags" placeholder="Tags"
                        class="input input-bordered w-full mb-4" />
                <!-- </div>
            </div> -->

            <select class="select select-bordered w-full mb-4">
                <option disabled selected>Category</option>
                <option>Electronics</option>
                <option>Clothing</option>
                <option>Books</option>
                <option>Home & Kitchen</option>
                <option>Sports & Outdoors</option>
            </select>
            <input type="file" class="file-input file-input-bordered w-full mb-4" />
            <div class="form-control">
                <label class="label cursor-pointer"></label>
                <span class="label-text">Available</span>
                <input type="checkbox" class="toggle toggle-primary" />
            </div>

            <button class="btn btn-primary">Save Product</button>
        </form>
    </div>
</template>

<script setup lang="ts">

</script>

<style scoped></style>