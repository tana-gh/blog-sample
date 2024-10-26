<script lang="ts">
  import { navigate } from 'svelte-routing'
  import { getPostUrl } from '../lib/api'
  import PostContent from '../components/PostContent.svelte'

  export let postId: string

  const fetchPost = (postId: string) => {
    const headers = new Headers()
    headers.append('Method', 'GET')
    headers.append('Access-Control-Allow-Headers', '*')
    headers.append('Accept', 'application/json')
    return fetch(getPostUrl(postId), { headers })
      .then(res => res.json())
  }
  $: postPromise = fetchPost(postId)
</script>

<main>
  {#await postPromise}
    <p>Loading...</p>
  {:then post}
    <PostContent post={post}/>
  {:catch error}
    { navigate('/404', { replace: true }) }
  {/await}
</main>

<style>
</style>
