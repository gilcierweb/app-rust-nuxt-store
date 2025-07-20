<template>
  <div>
    <div class="col-span-4 md:col-span-4">
      <h2 class="h2">Admin Dashboard</h2>
    </div>

    <div class="grid grid-cols-1 md:grid-cols-3 gap-4 p-4">
      <div class="stats max-sm:w-full">
        <div class="stat">
          <div class="avatar avatar-placeholder">
            <div class="bg-success/20 text-success size-10 rounded-full">
              <span class="icon-[tabler--package] size-6"></span>
            </div>
          </div>
          <div class="stat-value mb-1">Order</div>
          <div class="stat-title">7,500 of 10,000 orders</div>
          <div class="progress bg-success/10 h-2" role="progressbar" aria-label="Order Progressbar" aria-valuenow="75"
            aria-valuemin="0" aria-valuemax="100">
            <div class="progress-bar progress-success w-3/4"></div>
          </div>
        </div>
      </div>

      <div class="stats max-sm:w-full">
        <div class="stat">
          <div class="avatar avatar-placeholder">
            <div class="bg-warning/20 text-warning size-10 rounded-full">
              <span class="icon-[tabler--cash] size-6"></span>
            </div>
          </div>
          <div class="stat-value mb-1">Revenue</div>
          <div class="stat-title">$45,000 of $100,000</div>
          <div class="progress bg-warning/10 h-2" role="progressbar" aria-label="Revenue Progressbar" aria-valuenow="45"
            aria-valuemin="0" aria-valuemax="100">
            <div class="progress-bar progress-warning w-2/5"></div>
          </div>
        </div>
      </div>

      <div class="stats max-sm:w-full">
        <div class="stat">
          <div class="avatar avatar-placeholder">
            <div class="bg-error/20 text-error size-10 rounded-full">
              <span class="icon-[tabler--credit-card] size-6"></span>
            </div>
          </div>
          <div class="stat-value mb-1">Invoice</div>
          <div class="stat-title">$18,200 of $25,000</div>
          <div class="progress bg-error/10 h-2" role="progressbar" aria-label="Invoice Progressbar" aria-valuenow="73"
            aria-valuemin="0" aria-valuemax="100">
            <div class="progress-bar progress-error w-[73%]"></div>
          </div>
        </div>
      </div>
    </div>

<div class="grid grid-cols-1 md:grid-cols-3 gap-4 p-4">
    <div class="stats">
      <div class="stat">
        <div class="stat-figure text-base-content size-8">
          <span class="icon-[tabler--world] size-8"></span>
        </div>
        <div class="stat-title">Website Traffic</div>
        <div class="stat-value">32K</div>
        <div class="stat-desc">5% ↗︎ than last week</div>
      </div>

      <div class="stat">
        <div class="stat-figure text-base-content size-8">
          <span class="icon-[tabler--users-group] size-8"></span>
        </div>
        <div class="stat-title">New Signups</div>
        <div class="stat-value">1.2K</div>
        <div class="stat-desc">12% increase this month</div>
      </div>

      <div class="stat">
        <div class="stat-figure size-12">
          <div class="avatar">
            <div class="size-12 rounded-full">
              <img src="https://cdn.flyonui.com/fy-assets/avatar/avatar-2.png" alt="User Avatar" />
            </div>
          </div>
        </div>
        <div class="stat-value text-success">95%</div>
        <div class="stat-title">Customer Retention</div>
        <div class="stat-desc">Steady over last quarter</div>
      </div>
    </div>
    </div>

<div class="grid grid-cols-1 md:grid-cols-4 gap-4 p-4">
    <div class="stats">
      <div class="stat">
        <div class="stat-figure">
          <div class="avatar">
            <div class="size-12 rounded-full">
              <img src="https://cdn.flyonui.com/fy-assets/avatar/avatar-1.png" alt="User Avatar" />
            </div>
          </div>
        </div>
        <div class="stat-title">Total page views</div>
        <div class="stat-value">89,400</div>
        <div class="stat-desc">21% ↗︎ than last month</div>
      </div>
    </div>
    <div class="stats">
      <div class="stat">
        <div class="stat-figure">
          <div class="avatar">
            <div class="size-12 rounded-full">
              <img src="https://cdn.flyonui.com/fy-assets/avatar/avatar-1.png" alt="User Avatar" />
            </div>
          </div>
        </div>
        <div class="stat-title">Total page views</div>
        <div class="stat-value">89,400</div>
        <div class="stat-desc">21% ↗︎ than last month</div>
      </div>
    </div>
    <div class="stats">
      <div class="stat">
        <div class="stat-figure">
          <div class="avatar">
            <div class="size-12 rounded-full">
              <img src="https://cdn.flyonui.com/fy-assets/avatar/avatar-1.png" alt="User Avatar" />
            </div>
          </div>
        </div>
        <div class="stat-title">Total page views</div>
        <div class="stat-value">89,400</div>
        <div class="stat-desc">21% ↗︎ than last month</div>
      </div>
    </div>
    <div class="stats">
      <div class="stat">
        <div class="stat-figure">
          <div class="avatar">
            <div class="size-12 rounded-full">
              <img src="https://cdn.flyonui.com/fy-assets/avatar/avatar-1.png" alt="User Avatar" />
            </div>
          </div>
        </div>
        <div class="stat-title">Total page views</div>
        <div class="stat-value">89,400</div>
        <div class="stat-desc">21% ↗︎ than last month</div>
      </div>
    </div>
    </div>

<div class="grid grid-cols-1 md:grid-cols-3 gap-4 p-4">
    <div class="stats">
      <div class="stat place-items-center">
        <div class="stat-title">Total Sales</div>
        <div class="stat-value">52K</div>
        <div class="stat-desc">From Jan 1st to Feb 1st</div>
      </div>

      <div class="stat place-items-center">
        <div class="stat-title">Active Customers</div>
        <div class="stat-value">8,350</div>
        <div class="stat-desc">↗︎ 150 (1.8%)</div>
      </div>

      <div class="stat place-items-center">
        <div class="stat-title">New Signups</div>
        <div class="stat-value">2,400</div>
        <div class="stat-desc">↘︎ 180 (7%)</div>
      </div>
    </div>
    </div>

<div class="grid grid-cols-1 md:grid-cols-3 gap-4 p-4">
    <div class="stats max-md:stats-vertical">
      <div class="stat">
        <div class="stat-title">Subscription Plan</div>
        <div class="stat-value">Premium</div>
        <div class="stat-actions">
          <button class="btn btn-sm btn-primary btn-gradient">Upgrade Plan</button>
        </div>
      </div>

      <div class="stat">
        <div class="stat-title">Next Billing Date</div>
        <div class="stat-value">Oct 15, 2024</div>
        <div class="stat-actions flex flex-wrap gap-2">
          <button class="btn btn-sm btn-soft">View Details</button>
          <button class="btn btn-sm btn-warning btn-soft">Change Payment Method</button>
        </div>
      </div>
    </div>
    </div>

  </div>
</template>

<script lang="ts" setup>

</script>

<style></style>