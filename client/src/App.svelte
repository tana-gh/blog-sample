<script lang="ts">
  import { Link, Route, Router } from 'svelte-routing'
  import HomePage from './pages/HomePage.svelte'
  import PostPage from './pages/PostPage.svelte'
  import NotFoundPage from './pages/NotFoundPage.svelte'

  export let url = ''
</script>

<Router {url}>
  <nav>
    <Link to="/blog">Home</Link>
    <Link to="/blog/1">post 1</Link>
  </nav>
  <div>
    <Route path="/blog"><HomePage /></Route>
    <Route path="/blog/"><HomePage /></Route>
    <Route path="/blog/:postId" let:params><PostPage postId={params.postId} /></Route>
    <Route path="/blog/*"><NotFoundPage /></Route>
    <Route path="/404"><NotFoundPage /></Route>
  </div>
</Router>

<style>
</style>
