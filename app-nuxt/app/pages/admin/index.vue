<template>
  <div>
    <div class="col-span-4 md:col-span-4">
      <h2 class="h2">{{ $t('admin.dashboard.title') }}</h2>
    </div>

    <div class="grid grid-cols-1 md:grid-cols-3 gap-4 p-4">
      <div class="stats max-sm:w-full">
        <div class="stat">
          <div class="avatar avatar-placeholder">
            <div class="bg-success/20 text-success size-10 rounded-full">
              <span class="icon-[tabler--package] size-6"></span>
            </div>
          </div>
          <div class="stat-value mb-1">{{ $t('admin.dashboard.stats.order') }}</div>
          <div class="stat-title">{{ $t('admin.dashboard.stats.orderProgress', { current: '7,500', total: '10,000' }) }}</div>
          <div class="progress bg-success/10 h-2" role="progressbar" :aria-label="$t('admin.dashboard.stats.orderProgressbar')" aria-valuenow="75"
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
          <div class="stat-value mb-1">{{ $t('admin.dashboard.stats.revenue') }}</div>
          <div class="stat-title">{{ $t('admin.dashboard.stats.revenueProgress', { current: '45,000', total: '100,000' }) }}</div>
          <div class="progress bg-warning/10 h-2" role="progressbar" :aria-label="$t('admin.dashboard.stats.revenueProgressbar')" aria-valuenow="45"
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
          <div class="stat-value mb-1">{{ $t('admin.dashboard.stats.invoice') }}</div>
          <div class="stat-title">{{ $t('admin.dashboard.stats.invoiceProgress', { current: '18,200', total: '25,000' }) }}</div>
          <div class="progress bg-error/10 h-2" role="progressbar" :aria-label="$t('admin.dashboard.stats.invoiceProgressbar')" aria-valuenow="73"
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
        <div class="stat-title">{{ $t('admin.dashboard.stats.websiteTraffic') }}</div>
        <div class="stat-value">32K</div>
        <div class="stat-desc">{{ $t('admin.dashboard.stats.trafficChange', { percent: '5' }) }}</div>
      </div>

      <div class="stat">
        <div class="stat-figure text-base-content size-8">
          <span class="icon-[tabler--users-group] size-8"></span>
        </div>
        <div class="stat-title">{{ $t('admin.dashboard.stats.newSignups') }}</div>
        <div class="stat-value">1.2K</div>
        <div class="stat-desc">{{ $t('admin.dashboard.stats.signupsChange', { percent: '12' }) }}</div>
      </div>

      <div class="stat">
        <div class="stat-figure size-12">
          <div class="avatar">
            <div class="size-12 rounded-full">
              <img src="https://cdn.flyonui.com/fy-assets/avatar/avatar-2.png" :alt="$t('admin.navbar.profile.avatarAlt')" />
            </div>
          </div>
        </div>
        <div class="stat-value text-success">95%</div>
        <div class="stat-title">{{ $t('admin.dashboard.stats.customerRetention') }}</div>
        <div class="stat-desc">{{ $t('admin.dashboard.stats.retentionChange') }}</div>
      </div>
    </div>
    </div>

<div class="grid grid-cols-1 md:grid-cols-4 gap-4 p-4">
    <div class="stats">
      <div class="stat">
        <div class="stat-figure">
          <div class="avatar">
            <div class="size-12 rounded-full">
              <img src="https://cdn.flyonui.com/fy-assets/avatar/avatar-1.png" :alt="$t('admin.navbar.profile.avatarAlt')" />
            </div>
          </div>
        </div>
        <div class="stat-title">{{ $t('admin.dashboard.stats.totalPageViews') }}</div>
        <div class="stat-value">89,400</div>
        <div class="stat-desc">{{ $t('admin.dashboard.stats.pageViewsChange', { percent: '21' }) }}</div>
      </div>
    </div>
    <div class="stats">
      <div class="stat">
        <div class="stat-figure">
          <div class="avatar">
            <div class="size-12 rounded-full">
              <img src="https://cdn.flyonui.com/fy-assets/avatar/avatar-1.png" :alt="$t('admin.navbar.profile.avatarAlt')" />
            </div>
          </div>
        </div>
        <div class="stat-title">{{ $t('admin.dashboard.stats.totalPageViews') }}</div>
        <div class="stat-value">89,400</div>
        <div class="stat-desc">{{ $t('admin.dashboard.stats.pageViewsChange', { percent: '21' }) }}</div>
      </div>
    </div>
    <div class="stats">
      <div class="stat">
        <div class="stat-figure">
          <div class="avatar">
            <div class="size-12 rounded-full">
              <img src="https://cdn.flyonui.com/fy-assets/avatar/avatar-1.png" :alt="$t('admin.navbar.profile.avatarAlt')" />
            </div>
          </div>
        </div>
        <div class="stat-title">{{ $t('admin.dashboard.stats.totalPageViews') }}</div>
        <div class="stat-value">89,400</div>
        <div class="stat-desc">{{ $t('admin.dashboard.stats.pageViewsChange', { percent: '21' }) }}</div>
      </div>
    </div>
    <div class="stats">
      <div class="stat">
        <div class="stat-figure">
          <div class="avatar">
            <div class="size-12 rounded-full">
              <img src="https://cdn.flyonui.com/fy-assets/avatar/avatar-1.png" :alt="$t('admin.navbar.profile.avatarAlt')" />
            </div>
          </div>
        </div>
        <div class="stat-title">{{ $t('admin.dashboard.stats.totalPageViews') }}</div>
        <div class="stat-value">89,400</div>
        <div class="stat-desc">{{ $t('admin.dashboard.stats.pageViewsChange', { percent: '21' }) }}</div>
      </div>
    </div>
    </div>

<div class="grid grid-cols-1 md:grid-cols-3 gap-4 p-4">
    <div class="stats">
      <div class="stat place-items-center">
        <div class="stat-title">{{ $t('admin.dashboard.stats.totalSales') }}</div>
        <div class="stat-value">52K</div>
        <div class="stat-desc">{{ $t('admin.dashboard.stats.period', { start: 'Jan 1st', end: 'Feb 1st' }) }}</div>
      </div>

      <div class="stat place-items-center">
        <div class="stat-title">{{ $t('admin.dashboard.stats.activeCustomers') }}</div>
        <div class="stat-value">8,350</div>
        <div class="stat-desc">↗︎ 150 (1.8%)</div>
      </div>

      <div class="stat place-items-center">
        <div class="stat-title">{{ $t('admin.dashboard.stats.newSignups') }}</div>
        <div class="stat-value">2,400</div>
        <div class="stat-desc">↘︎ 180 (7%)</div>
      </div>
    </div>
    </div>

<div class="grid grid-cols-1 md:grid-cols-3 gap-4 p-4">
    <div class="stats max-md:stats-vertical">
      <div class="stat">
        <div class="stat-title">{{ $t('admin.dashboard.stats.subscription') }}</div>
        <div class="stat-value">{{ $t('admin.dashboard.stats.subscriptionPlan') }}</div>
        <div class="stat-actions">
          <button class="btn btn-sm btn-primary btn-gradient">{{ $t('admin.dashboard.stats.upgradePlan') }}</button>
        </div>
      </div>

      <div class="stat">
        <div class="stat-title">{{ $t('admin.dashboard.stats.nextBilling') }}</div>
        <div class="stat-value">{{ $t('admin.dashboard.stats.nextBillingDate') }}</div>
        <div class="stat-actions flex flex-wrap gap-2">
          <button class="btn btn-sm btn-soft">{{ $t('admin.dashboard.stats.viewDetails') }}</button>
          <button class="btn btn-sm btn-warning btn-soft">{{ $t('admin.dashboard.stats.changePayment') }}</button>
        </div>
      </div>
    </div>
    </div>

  </div>
</template>

<script lang="ts" setup>

</script>

<style></style>